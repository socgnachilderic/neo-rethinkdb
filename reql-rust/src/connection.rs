use async_net::TcpStream;
use dashmap::DashMap;
use futures::TryFutureExt;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use ql2::query::QueryType;
use ql2::response::ResponseType;
use serde_json::json;
use tokio::time;
use std::borrow::Cow;
use std::ops::Drop;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use tracing::trace;
use reql_rust_types::ServerInfo;

use crate::proto::{Payload, Query};
use crate::{Result, err, ReqlDriverError, r};
use super::cmd::run::Response;
use super::cmd::StaticString;

type Sender = UnboundedSender<Result<(ResponseType, Response)>>;
type Receiver = UnboundedReceiver<Result<(ResponseType, Response)>>;

#[derive(Debug)]
pub(crate) struct InnerSession {
    pub(crate) db: Mutex<Cow<'static, str>>,
    pub(crate) stream: Mutex<TcpStream>,
    pub(crate) channels: DashMap<u64, Sender>,
    pub(crate) token: AtomicU64,
    pub(crate) broken: AtomicBool,
    pub(crate) change_feed: AtomicBool,
}

impl InnerSession {
    pub(crate) fn token(&self) -> u64 {
        let token = self
            .token
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1))
            .unwrap();
        if token == u64::MAX {
            self.mark_broken();
        }
        token
    }

    pub(crate) fn mark_broken(&self) {
        self.broken.store(true, Ordering::SeqCst);
    }

    pub(crate) fn broken(&self) -> Result<()> {
        if self.broken.load(Ordering::SeqCst) {
            return Err(err::ReqlDriverError::ConnectionBroken.into());
        }
        Ok(())
    }

    pub(crate) fn mark_change_feed(&self) {
        self.change_feed.store(true, Ordering::SeqCst);
    }

    pub(crate) fn unmark_change_feed(&self) {
        self.change_feed.store(false, Ordering::SeqCst);
    }

    pub(crate) fn is_change_feed(&self) -> bool {
        self.change_feed.load(Ordering::SeqCst)
    }

    pub(crate) fn change_feed(&self) -> Result<()> {
        if self.change_feed.load(Ordering::SeqCst) {
            return Err(err::ReqlDriverError::ConnectionLocked.into());
        }
        Ok(())
    }
}

/// The connection object returned by `r.connection()`
#[derive(Debug, Clone)]
pub struct Session {
    pub(crate) inner: Arc<InnerSession>,
}

impl Session {
    #[doc(hidden)]
    pub fn connection(&self) -> Result<Connection> {
        self.inner.broken()?;
        self.inner.change_feed()?;
        let token = self.inner.token();
        let (tx, rx) = mpsc::unbounded();
        self.inner.channels.insert(token, tx);
        Ok(Connection::new(self.clone(), rx, token))
    }

    /// Close and reopen a connection.
    /// Closing a connection normally waits until all outstanding requests have finished 
    /// and then frees any open resources associated with the connection. 
    /// By passing `false` as an optional boolean argument to `reconnect`, 
    /// the connection will be closed immediately, possibly aborting any outstanding noreply writes. 
    /// A optional second argument is a (`Option<std::time::Duration>`) timeout indicating how long you would like 
    /// `reconnect` to wait before closing the existing connection.
    /// 
    /// A noreply query is executed by passing the `noreply` option to the
    /// [run](crate::Command::run) command, indicating that `run()` should not
    /// wait for the query to complete before returning. You may also
    /// explicitly wait for a noreply query to complete by using the
    /// [noreply_wait](crate::Session::noreply_wait) command.
    /// 
    /// ## Example
    /// 
    /// Cancel outstanding requests/queries that are no longer needed.
    /// 
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.reconnect(false, None).await
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Wait up for outstanding requests to finish before reconnecting.
    /// 
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.reconnect(true, None).await
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Cancel outstanding requests/queries that are no longer needed after a timeout.
    /// 
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.reconnect(false, Some(std::time::Duration::from_secs(5))).await
    /// }
    /// ```
    pub async fn reconnect(&self, noreply_wait: bool, timeout: Option<std::time::Duration>) -> Result<()> {
        let future = self.close(noreply_wait).and_then(|_| async {self.connection()});
    
        if let Some(timeout) = timeout {
            time::timeout(timeout, future).await.unwrap()?;
        } else {
            future.await?;
        }

        Ok(())
    }

    /// Change the default database on this connection
    ///
    /// ## Example
    ///
    /// Change the default database so that we don’t need to specify the
    /// database when referencing a table.
    ///
    /// ```
    /// use reql_rust::{r, Result};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut session = r.connection().connect().await?;
    ///     session.use_("marvel").await;
    ///     r.table("heroes").run::<_, serde_json::Value>(&session).try_next().await?; // refers to r.db("marvel").table("heroes")
    ///     Ok(())
    ///}
    /// ```
    ///
    /// ## Related commands
    /// * [connection](r::connection)
    /// * [close](Connection::close)
    pub async fn use_(&mut self, db_name: &'static str) {
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
    /// We have previously run queries with [noreply](crate::cmd::run::Options::noreply())
    /// set to `true`. Now wait until the server has processed them.
    ///
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.noreply_wait().await
    /// }
    /// ```
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

    ///
     /// Return information about the server being used by a connection.
     /// 
     /// The server command returns `ServerInfo` struct with two or three fields:
     /// 
     /// - `id` : the UUID of the server the client is connected to.
     /// - `proxy` : a boolean indicating whether the server is a [RethinkDB proxy node](https://rethinkdb.com/docs/sharding-and-replication/#running-a-proxy-node).
     /// - `name` : the server name. If `proxy` is `true`, this field will not be returned.
     /// 
     /// ## Example
     /// 
     /// Return server information.
     /// 
     /// ```
     /// use reql_rust::{r, Result, types::ServerInfo};
     /// 
     /// async fn example() -> Result<ServerInfo> {
     ///     let session = r.connection().connect().await?;
     ///     session.server().await
     /// }
     /// ```
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
            .ok_or_else(|| ReqlDriverError::Other("server info is empty".into()))?;
        Ok(info)
    }

    /// Close an open connection
    ///
    /// Closing a connection normally waits until all outstanding requests have
    /// finished and then frees any open resources associated with the
    /// connection. By passing `SkipNoreplyWait` as the argument, the connection
    /// will be closed immediately, possibly aborting any outstanding noreply
    /// writes.
    ///
    /// A noreply query is executed by passing the `noreply` option to the
    /// [run](crate::Command::run) command, indicating that `run()` should not
    /// wait for the query to complete before returning. You may also
    /// explicitly wait for a noreply query to complete by using the
    /// [noreply_wait](crate::Session::noreply_wait) command.
    ///
    ///
    /// ## Example
    ///
    /// Close an open connection, waiting for noreply writes to finish.
    ///
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.close(true).await
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Close an open connection immediately.
    ///
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     session.close(false).await
    /// }
    /// ```
    ///
    /// ## Related commands
    ///
    /// * [connection](r::connection)
    /// * [use_](crate::Session::use_)
    pub async fn close(&self, noreply_wait: bool) -> Result<()> {
        self.connection()?.close(noreply_wait).await
    }

    #[doc(hidden)]
    pub fn is_broken(&self) -> bool {
        self.inner.broken.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub(crate) session: Session,
    pub(crate) rx: Arc<Mutex<Receiver>>,
    pub(crate) token: u64,
    pub(crate) closed: Arc<AtomicBool>,
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

    pub async fn close(&mut self, noreply_wait: bool) -> Result<()> {
        if !self.session.inner.is_change_feed() {
            trace!(
                "ignoring conn.close() called on a normal connection; token: {}",
                self.token
            );
            return Ok(());
        }

        self.set_closed(true);

        let arg = if noreply_wait {
            Some(r.expr(json!({ "noreply": false })))
        } else {
            None
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

    pub(crate) fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub(crate) fn set_closed(&self, closed: bool) {
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
