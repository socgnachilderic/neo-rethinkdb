//! ReQL is the RethinkDB query language. It offers a very powerful and
//! convenient way to manipulate JSON documents.
//!
//! # Start the server #
//!
//! ## Linux and OS X ##
//!
//! Start the server from a terminal window.
//!
//! ```bash
//! $ rethinkdb
//! ```
//!
//! ## Windows ##
//!
//! Start the server from the Windows command prompt.
//!
//! ```bash
//! C:\Path\To\RethinkDB\>rethinkdb.exe
//! ```
//!
//! # Import the driver #
//!
//! First, make sure you have `protoc` installed and in your `PATH`. See
//! [`prost-build` documentation](https://docs.rs/prost-build/0.7.0/prost_build/#sourcing-protoc)
//! for more details if it fails to compile.
//!
//! Add this crate (`reql`) and the `futures` crate to your dependencies in `Cargo.toml`.
//!
//! Now import the RethinkDB driver:
//!
//! ```
//! use reql_rust::r;
//! ```
//!
//! You can now access RethinkDB commands through the [`r` struct](r).
//!
//! # Open a connection #
//!
//! When you first start RethinkDB, the server opens a port for the client
//! drivers (`28015` by default). Let's open a connection:
//!
//! ```
//! use reql_rust::r;
//!
//! # async fn example() -> reql_rust::Result<()> {
//! let session = r.connect(()).await?;
//! # Ok(()) };
//! ```
//!
//! The variable `connection` is now initialized and we can run queries.
//!
//! # Send a query to the database #
//!
//! ```
//! # reql_rust::example(|r, conn| async_stream::stream! {
//! r.expr("Hello world!").run(conn)
//! # });
//! ```
//!
//! [See the `r` struct for more available commands](r)

#![allow(clippy::wrong_self_convention)]

pub mod cmd;
mod err;
mod proto;

use async_net::TcpStream;
use cmd::run::Response;
use cmd::StaticString;
use dashmap::DashMap;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use proto::{Payload, Query};
use ql2::query::QueryType;
use ql2::response::ResponseType;
use ql2::term::TermType;
use serde_json::json;
use std::borrow::Cow;
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tracing::trace;
use reql_rust_types::ServerInfo;

#[doc(hidden)]
pub use cmd::func::Func;
pub use err::*;
pub use proto::Command;
pub use reql_rust_macros::func;
#[doc(inline)]
pub use reql_rust_types as types;

#[doc(hidden)]
pub static VAR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[doc(hidden)]
pub fn var_counter() -> u64 {
    VAR_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[cfg(test)]
fn current_counter() -> u64 {
    VAR_COUNTER.load(Ordering::SeqCst)
}

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

type Sender = UnboundedSender<Result<(ResponseType, Response)>>;
type Receiver = UnboundedReceiver<Result<(ResponseType, Response)>>;

#[derive(Debug)]
struct InnerSession {
    db: Mutex<Cow<'static, str>>,
    stream: Mutex<TcpStream>,
    channels: DashMap<u64, Sender>,
    token: AtomicU64,
    broken: AtomicBool,
    change_feed: AtomicBool,
}

impl InnerSession {
    fn token(&self) -> u64 {
        let token = self
            .token
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1))
            .unwrap();
        if token == u64::MAX {
            self.mark_broken();
        }
        token
    }

    fn mark_broken(&self) {
        self.broken.store(true, Ordering::SeqCst);
    }

    fn broken(&self) -> Result<()> {
        if self.broken.load(Ordering::SeqCst) {
            return Err(err::Driver::ConnectionBroken.into());
        }
        Ok(())
    }

    fn mark_change_feed(&self) {
        self.change_feed.store(true, Ordering::SeqCst);
    }

    fn unmark_change_feed(&self) {
        self.change_feed.store(false, Ordering::SeqCst);
    }

    fn is_change_feed(&self) -> bool {
        self.change_feed.load(Ordering::SeqCst)
    }

    fn change_feed(&self) -> Result<()> {
        if self.change_feed.load(Ordering::SeqCst) {
            return Err(err::Driver::ConnectionLocked.into());
        }
        Ok(())
    }
}

/// The connection object returned by `r.connect()`
#[derive(Debug, Clone)]
pub struct Session {
    inner: Arc<InnerSession>,
}

impl Session {
    pub fn connection(&self) -> Result<Connection> {
        self.inner.broken()?;
        self.inner.change_feed()?;
        let token = self.inner.token();
        let (tx, rx) = mpsc::unbounded();
        self.inner.channels.insert(token, tx);
        Ok(Connection::new(self.clone(), rx, token))
    }

    /// Change the default database on this connection
    ///
    /// ## Example
    ///
    /// Change the default database so that we don’t need to specify the
    /// database when referencing a table.
    ///
    /// ```
    /// # reql_rust::example(|r, conn| async_stream::stream! {
    /// conn.use_("marvel").await;
    /// r.table("heroes").run(conn) // refers to r.db("marvel").table("heroes")
    /// # });
    /// ```
    ///
    /// ## Related commands
    /// * [connect](r::connect)
    /// * [close](Connection::close)
    pub async fn use_<T>(&mut self, db_name: T)
    where
        T: StaticString,
    {
        *self.inner.db.lock().await = db_name.static_string();
    }

    /// Ensures that previous queries with the `noreply` flag have been
    /// processed by the server
    ///
    /// Note that this guarantee only applies to queries run on the given
    /// connection.
    ///
    /// ## Example
    ///
    /// We have previously run queries with [noreply](cmd::run::Options::noreply())
    /// set to `true`. Now wait until the server has processed them.
    ///
    /// ```
    /// # async fn example() -> reql_rust::Result<()> {
    /// # let session = reql_rust::r.connect(()).await?;
    /// session.noreply_wait().await
    /// # }
    /// ```
    ///
    pub async fn noreply_wait(&self) -> Result<()> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::NoreplyWait, None, Default::default());
        trace!(
            "waiting for noreply operations to finish; token: {}",
            conn.token
        );
        let (typ, _) = conn.request(&payload, false).await?;
        trace!(
            "session.noreply_wait() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        Ok(())
    }

    pub async fn server(&self) -> Result<ServerInfo> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::ServerInfo, None, Default::default());
        trace!("retrieving server information; token: {}", conn.token);
        let (typ, resp) = conn.request(&payload, false).await?;
        trace!(
            "session.server() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        let mut vec = serde_json::from_value::<Vec<ServerInfo>>(resp.r)?;
        let info = vec
            .pop()
            .ok_or_else(|| Driver::Other("server info is empty".into()))?;
        Ok(info)
    }

    #[doc(hidden)]
    pub fn is_broken(&self) -> bool {
        self.inner.broken.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    session: Session,
    rx: Arc<Mutex<Receiver>>,
    token: u64,
    closed: Arc<AtomicBool>,
}

impl Connection {
    fn new(session: Session, rx: Receiver, token: u64) -> Connection {
        Connection {
            session,
            token,
            rx: Arc::new(Mutex::new(rx)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Close an open connection
    ///
    /// ## Example
    ///
    /// Close an open connection, waiting for noreply writes to finish.
    ///
    /// ```
    /// # async fn example() -> reql_rust::Result<()> {
    /// # let session = reql_rust::r.connect(()).await?;
    /// # let mut conn = session.connection()?;
    /// conn.close(()).await
    /// # }
    /// ```
    ///
    /// [Read more about this command →](cmd::close)
    pub async fn close<T>(&mut self, arg: T) -> Result<()>
    where
        T: cmd::close::Arg,
    {
        if !self.session.inner.is_change_feed() {
            trace!(
                "ignoring conn.close() called on a normal connection; token: {}",
                self.token
            );
            return Ok(());
        }
        self.set_closed(true);
        let arg = if arg.noreply_wait() {
            None
        } else {
            Some(r.expr(json!({ "noreply": false })))
        };
        let payload = Payload(QueryType::Stop, arg.as_ref().map(Query), Default::default());
        trace!("closing a changefeed; token: {}", self.token);
        let (typ, _) = self.request(&payload, false).await?;
        self.session.inner.unmark_change_feed();
        trace!(
            "conn.close() run; token: {}, response type: {:?}",
            self.token,
            typ,
        );
        Ok(())
    }

    fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    fn set_closed(&self, closed: bool) {
        self.closed.store(closed, Ordering::SeqCst);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.session.inner.channels.remove(&self.token);
        if self.session.inner.is_change_feed() {
            self.session.inner.unmark_change_feed();
        }
    }
}

/// The top-level ReQL namespace
///
/// # Example
///
/// Set up your top-level namespace.
///
/// ```
/// use reql_rust::r;
/// ```
#[allow(non_camel_case_types)]
pub struct r;

impl r {
    /// Create a new connection to the database server
    ///
    /// # Example
    ///
    /// Open a connection using the default host and port, specifying the default database.
    ///
    /// ```
    /// use reql_rust::{r, cmd::connect::Options};
    ///
    /// # async fn example() -> reql_rust::Result<()> {
    /// let session = r.connect(Options::new().db("marvel")).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// Read more about this command [connect](cmd::connect)
    pub async fn connect<T>(self, options: T) -> Result<Session>
    where
        T: cmd::connect::Arg,
    {
        cmd::connect::new(options.into_connect_opts()).await
    }

    pub fn db_create<T>(self, arg: T) -> Command
    where
        T: cmd::db_create::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn db_drop<T>(self, arg: T) -> Command
    where
        T: cmd::db_drop::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn db_list(self) -> Command {
        Command::new(TermType::DbList)
    }

    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the
    /// query will run against the default database for the connection,
    /// specified in the `db` argument to [connect](r::connect).
    ///
    /// # Examples
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```
    /// # reql_rust::example(|r, conn| async_stream::stream! {
    /// r.db("heroes").table("marvel").run(conn)
    /// # });
    /// ```
    pub fn db<T>(self, arg: T) -> Command
    where
        T: cmd::db::Arg,
    {
        arg.arg().into_cmd()
    }

    /// See [Command::table_create]
    pub fn table_create<T>(self, arg: T) -> Command
    where
        T: cmd::table_create::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn table<T>(self, arg: T) -> Command
    where
        T: cmd::table::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn map<T>(self, arg: T) -> Command
    where
        T: cmd::map::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn union<T>(self, arg: T) -> Command
    where
        T: cmd::union::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn group<T>(self, arg: T) -> Command
    where
        T: cmd::group::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn reduce<T>(self, arg: T) -> Command
    where
        T: cmd::reduce::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn count<T>(self, arg: T) -> Command
    where
        T: cmd::count::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn sum<T>(self, arg: T) -> Command
    where
        T: cmd::sum::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn avg<T>(self, arg: T) -> Command
    where
        T: cmd::avg::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn min<T>(self, arg: T) -> Command
    where
        T: cmd::min::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn max<T>(self, arg: T) -> Command
    where
        T: cmd::max::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn distinct<T>(self, arg: T) -> Command
    where
        T: cmd::distinct::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn contains<T>(self, arg: T) -> Command
    where
        T: cmd::contains::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn literal<T>(self, arg: T) -> Command
    where
        T: cmd::literal::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn object<T>(self, arg: T) -> Command
    where
        T: cmd::object::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn random<T>(self, arg: T) -> Command
    where
        T: cmd::random::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn round<T>(self, arg: T) -> Command
    where
        T: cmd::round::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn ceil<T>(self, arg: T) -> Command
    where
        T: cmd::ceil::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn floor<T>(self, arg: T) -> Command
    where
        T: cmd::floor::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn now(self) -> Command {
        Command::new(TermType::Now)
    }

    pub fn time<T>(self, arg: T) -> Command
    where
        T: cmd::time::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn epoch_time<T>(self, arg: T) -> Command
    where
        T: cmd::epoch_time::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn iso8601<T>(self, arg: T) -> Command
    where
        T: cmd::iso8601::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn do_<T>(self, arg: T) -> Command
    where
        T: cmd::do_::Arg,
    {
        arg.arg(None).into_cmd()
    }

    pub fn branch<T>(self, arg: T) -> Command
    where
        T: cmd::branch::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn range<T>(self, arg: T) -> Command
    where
        T: cmd::range::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn error<T>(self, arg: T) -> Command
    where
        T: cmd::error::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn expr<T>(self, arg: T) -> Command
    where
        T: cmd::expr::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn js<T>(self, arg: T) -> Command
    where
        T: cmd::js::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn info<T>(self, arg: T) -> Command
    where
        T: cmd::info::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn json<T>(self, arg: T) -> Command
    where
        T: cmd::json::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn http<T>(self, arg: T) -> Command
    where
        T: cmd::http::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn uuid<T>(self, arg: T) -> Command
    where
        T: cmd::uuid::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn circle<T>(self, arg: T) -> Command
    where
        T: cmd::circle::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn distance<T>(self, arg: T) -> Command
    where
        T: cmd::distance::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn geojson<T>(self, arg: T) -> Command
    where
        T: cmd::geojson::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn intersects<T>(self, arg: T) -> Command
    where
        T: cmd::intersects::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn line<T>(self, arg: T) -> Command
    where
        T: cmd::line::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn point<T>(self, arg: T) -> Command
    where
        T: cmd::point::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn polygon<T>(self, arg: T) -> Command
    where
        T: cmd::polygon::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn grant<T>(self, arg: T) -> Command
    where
        T: cmd::grant::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn wait<T>(self, arg: T) -> Command
    where
        T: cmd::wait::Arg,
    {
        arg.arg().into_cmd()
    }

    pub fn asc<T>(self, arg: T) -> cmd::asc::Asc
    where
        T: cmd::asc::Arg,
    {
        cmd::asc::Asc(arg.arg().into_cmd())
    }

    pub fn desc<T>(self, arg: T) -> cmd::desc::Desc
    where
        T: cmd::desc::Arg,
    {
        cmd::desc::Desc(arg.arg().into_cmd())
    }

    pub fn index<T>(self, arg: T) -> cmd::index::Index
    where
        T: cmd::index::Arg,
    {
        cmd::index::Index(arg.arg().into_cmd())
    }

    pub fn args<T>(self, arg: T) -> cmd::args::Args<T> {
        cmd::args::Args(arg)
    }
}

// Helper for making writing examples less verbose
#[doc(hidden)]
pub fn example<'a, Q, F, S>(_query: Q)
where
    Q: FnOnce(r, &'a mut Session) -> async_stream::AsyncStream<(), F>,
    F: futures::Future<Output = S>,
    S: futures::Stream<Item = Result<serde_json::Value>>,
{
}
