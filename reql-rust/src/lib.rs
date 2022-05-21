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
//! let session = r.connection().connect().await?;
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
pub mod connection;
pub mod prelude;
pub mod types;
mod err;
mod proto;
mod constants;

use cmd::{db_create::DbCreate, db_drop::DbDrop, db_list::DbList, table_create::TableCreateBuilder, table_drop::TableDropBuilder, table_list::TableListBuilder};
use ql2::term::TermType;

use std::sync::atomic::{AtomicU64, Ordering};
pub use prelude::Func;

pub use err::*;
pub use connection::*;
pub use proto::Command;


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
pub type Result<T> = std::result::Result<T, ReqlError>;

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
    /// Create a new connection to the database server.
    /// [connection](cmd::connect::ConnectionBuilder::connection) returns a connection builder with the following methods:
    /// - [with_host(&'static str)](cmd::connect::ConnectionBuilder::with_host): the host to connect to (default `localhost`)
    /// - [with_port(value: u16)](cmd::connect::ConnectionBuilder::with_port): the port to connect on (default `28015`).
    /// - [with_db(value: &'static str)](cmd::connect::ConnectionBuilder::with_db): the default database (default `test`).
    /// - [with_user(username: &'static str, password: &'static str)](cmd::connect::ConnectionBuilder::with_user): the user account and password to connect as (default `"admin", ""`).
    /// 
    ///
    /// # Example
    ///
    /// Open a connection using the default host and port, specifying the default database.
    ///
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection().connect().await?;
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Example
    /// 
    /// Open a new connection, specifying parameters.
    /// 
    /// ```
    /// async fn example() -> reql_rust::Result<()> {
    ///     let session = reql_rust::r.connection()
    ///         .with_host("localhost")
    ///         .with_port(28015)
    ///         .with_db("marvel")
    ///         .connect().await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Read more about this command [connect](cmd::connect::ConnectionBuilder)
    pub fn connection(self) -> cmd::connect::ConnectionBuilder {
        cmd::connect::ConnectionBuilder::connection()
    }

    /// Create a database. A RethinkDB database is a collection of tables, similar to relational databases.
    /// 
    /// If successful, the command returns an object with two fields:
    /// * `dbs_created` : always `1`.
    /// * `config_changes` : a list containing one object with two fields, `old_val` and `new_val` :
    ///     - `old_val` : always `None`.
    ///     - `new_val` : the database’s new [config](https://rethinkdb.com/api/java/config) value.
    /// 
    /// If a database with the same name already exists, the command throws `ReqlRuntimeError`.
    /// 
    /// Note: Only alphanumeric characters and underscores are valid for the database name.
    /// 
    /// # Example
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::{DbCreateReturnType};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _val: Option<DbCreateReturnType> = r.db_create("superheroes")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// Return:
    /// ```text
    /// Some(
    ///     DbCreateReturnType {
    ///         config_changes: [
    ///             DbConfigChange {
    ///                 new_val: Some(
    ///                     DbConfigChangeValue {
    ///                         id: "e4689cfc-e903-4532-a0e6-2d6797a43f07",
    ///                         name: "superheroes",
    ///                     },
    ///                 ),
    ///                 old_val: None,
    ///             },
    ///         ],
    ///         dbs_created: 1,
    ///     },
    /// )
    /// ```
    pub fn db_create(self, db_name: &'static str) -> Command {
        DbCreate::new(db_name)
    }

    /// Drop a database. The database, all its tables, and corresponding data will be deleted.
    /// 
    /// If successful, the command returns an object with two fields:
    /// 
    /// * `dbs_dropped` : 1.
    /// * `tables_dropped` : the number of tables in the dropped database.
    /// * `config_changes` : a list containing one two-field object, `old_val` and `new_val` :
    ///     - `old_val` : the database’s original [config](https://rethinkdb.com/api/java/config) value.
    ///     - `new_val` : always `None`.
    /// 
    /// If the given database does not exist, the command throws ReqlRuntimeError.
    /// 
    /// # Example
    /// 
    /// Drop a database named ‘superheroes’.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::{DbDropReturnType};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _val: Option<DbDropReturnType> = r.db_drop("superheroes")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// Return:
    /// ```text
    /// Some(
    ///     DbDropReturnType {
    ///         config_changes: [
    ///             DbConfigChange {
    ///                 old_val: Some(
    ///                     DbConfigChangeValue {
    ///                         id: "e4689cfc-e903-4532-a0e6-2d6797a43f07",
    ///                         name: "superheroes",
    ///                     },
    ///                 ),
    ///                 new_val: None,
    ///             },
    ///         ],
    ///         tables_dropped: 3,
    ///         dbs_dropped: 1,
    ///     },
    /// )
    /// ```
    pub fn db_drop(self, db_name: &'static str) -> Command {
        DbDrop::new(db_name)
    }

    /// List all database names in the cluster. The result is a list of strings.
    /// 
    /// Example
    /// 
    /// List all databases.
    /// 
    /// ```
    /// use std::borrow::Cow;
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _val: Option<Vec<Cow<'static, str>>> = r.db_list()
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn db_list(self) -> Command {
        DbList::new()
    }

    /// Reference a database
    ///
    /// The `db` command is optional. If it is not present in a query, the
    /// query will run against the default database for the connection,
    /// specified in the `db` argument to [connection](r::connection).
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
    pub fn db(self, arg: impl cmd::db::Arg) -> Command {
        arg.arg().into_cmd()
    }

    /// Create a table
    ///
    /// A RethinkDB table is a collection of JSON documents.
    /// 
    /// If successful, the command returns an object with two fields:
    /// * `tables_created` : always `1`.
    /// * `config_changes` : a list containing one two-field object, `old_val` and `new_val` :
    ///     - `old_val` : always `None` .
    ///     - `new_val` : the table’s new [config](https://rethinkdb.com/api/java/config) value.
    /// 
    /// If a table with the same name already exists, the command throws ReqlRuntimeError.
    /// 
    /// # Note
    /// 
    /// Only alphanumeric characters and underscores are valid for the table name.
    /// 
    /// ```text
    /// Invoking tableCreate without specifying a database using db creates a 
    /// table in the database specified in connect, or test if no database was specified.
    /// ```
    /// 
    /// When creating a table, [TableCreateBuild](cmd::table_create::TableCreateBuilder) 
    /// returned you can specify the options with following method:
    /// * [with_primary_key(&'static str)](cmd::table_create::TableCreateBuilder::with_primary_key) :
    /// the name of the primary key. The default primary key is `id`.
    /// * [with_durability(types::Durability)](cmd::table_create::TableCreateBuilder::with_durability) :
    /// if set to `Durability::Soft`, writes will be acknowledged by the server immediately and flushed to disk in 
    /// the background. The default is `Durability::Hard`: acknowledgment of writes happens after data has been 
    /// written to disk
    /// * [with_shards(u8)](cmd::table_create::TableCreateBuilder::with_shards) :
    /// the number of shards, an integer from 1-64. Defaults to 1.
    /// * [with_replicas(types::Replicas)](cmd::table_create::TableCreateBuilder::with_replicas) :
    /// either an integer or a mapping object. Defaults to `Replicas::Int(1)`.
    ///     - If `replicas` is an `Replicas::Int`, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    ///     - If `replicas` is an `Replicas::Map`, t specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}` .
    ///
    /// Tables will be available for writing when the command returns.
    /// 
    /// # Example
    ///
    /// Create a table named "dc_universe" with the default settings.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table_create("dc_universe")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// Return :
    /// 
    /// ```text
    /// Some(
    ///     TableCreateReturnType {
    ///         config_changes: [
    ///             DbConfigChange {
    ///                 new_val: Some(
    ///                     DbConfigChangeValue {
    ///                         id: "20ea60d4-3b76-4817-8828-98a236df0297",
    ///                         name: "dc_universe",
    ///                         db: Some(
    ///                             "test",
    ///                         ),
    ///                         durability: Some(
    ///                             Hard,
    ///                         ),
    ///                         indexes: Some(
    ///                             [],
    ///                         ),
    ///                         primary_key: Some(
    ///                             "id",
    ///                         ),
    ///                         shards: Some(
    ///                             [
    ///                                 ShardType {
    ///                                     primary_replica: "rethinkdb_srv1",
    ///                                     replicas: [
    ///                                         "rethinkdb_srv1",
    ///                                         "rethinkdb_srv2"
    ///                                     ],
    ///                                 },
    ///                             ],
    ///                         ),
    ///                         write_acks: Some(
    ///                             Majority,
    ///                         ),
    ///                         write_hook: None,
    ///                     },
    ///                 ),
    ///                 old_val: None,
    ///             },
    ///         ],
    ///         tables_created: 1,
    ///     },
    /// )
    /// ```
    /// 
    /// # Example
    /// 
    /// Create a table named ‘dc_universe’ using the field ‘name’ as primary key.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table_create("dc_universe")
    ///         .with_primary_key("name")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Example
    /// 
    /// Create a table set up for two shards and three replicas per shard. This requires three available servers.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::Replicas;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table_create("dc_universe")
    ///         .with_shards(2)
    ///         .with_replicas(Replicas::Int(3))
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn table_create(self, table_name: &'static str) -> TableCreateBuilder {
        TableCreateBuilder::new(table_name)
    }

    /// Drop a table from a default database. The table and all its data will be deleted.
    /// 
    /// If successful, the command returns an object with two fields:
    /// * `tables_dropped` : always `1`.
    /// * `config_changes` : a list containing one two-field object, `old_val` and `new_val` :
    ///     - `old_val` : the dropped table”s [config](https://rethinkdb.com/api/java/config) value.
    ///     - `new_val` : always `null`.
    /// 
    /// If the given table does not exist in the database, the command throws `ReqlRuntimeError`.
    /// 
    /// ## Example
    /// 
    /// Drop a table named “dc_universe”.
    /// 
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table_drop("dc_universe")
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Return
    /// 
    /// ```text
    /// Some(
    ///     TableDropReturnType {
    ///         config_changes: [
    ///             DbConfigChange {
    ///                 new_val: None,
    ///                 old_val: Some(
    ///                     DbConfigChangeValue {
    ///                         id: "1bdc3c9c-e2ea-42d5-8c70-61dee9cb3f9d",
    ///                         name: "dc_universe",
    ///                         db: Some(
    ///                             "test",
    ///                         ),
    ///                         durability: Some(
    ///                             Hard,
    ///                         ),
    ///                         indexes: Some(
    ///                             [],
    ///                         ),
    ///                         primary_key: Some(
    ///                             "id",
    ///                         ),
    ///                         shards: Some(
    ///                             [
    ///                                 ShardType {
    ///                                     primary_replica: "00_11_22_33_44_55_pha",
    ///                                     replicas: [
    ///                                         "00_11_22_33_44_55_pha",
    ///                                     ],
    ///                                 },
    ///                             ],
    ///                         ),
    ///                         write_acks: Some(
    ///                             Majority,
    ///                         ),
    ///                         write_hook: None,
    ///                     },
    ///                 ),
    ///             },
    ///         ],
    ///         tables_dropped: 1,
    ///     },
    /// )
    /// ```
    pub fn table_drop(self, table_name: &'static str) -> TableDropBuilder {
        TableDropBuilder::new(table_name)
    }

    /// List all table names in a default database. The result is a list of strings.
    /// 
    /// # Example
    /// 
    /// List all tables of the default database (‘test’).
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table_list()
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn table_list(self) -> TableListBuilder {
        TableListBuilder::new()
    }

    pub fn table(self, arg: impl cmd::table::Arg) -> Command {
        arg.arg().into_cmd()
    }

    pub fn map(self, arg: impl cmd::map::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn union(self, arg: impl cmd::union::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn group(self, arg: impl cmd::group::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn reduce(self, arg: impl cmd::reduce::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn count(self, arg: impl cmd::count::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn sum(self, arg: impl cmd::sum::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn avg(self, arg: impl cmd::avg::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn min(self, arg: impl cmd::min::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn max(self, arg: impl cmd::max::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn distinct(self, arg: impl cmd::distinct::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn contains(self, arg: impl cmd::contains::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn literal(self, arg: impl cmd::literal::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn object(self, arg: impl cmd::object::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn random(self, arg: impl cmd::random::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn round(self, arg: impl cmd::round::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn ceil(self, arg: impl cmd::ceil::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn floor(self, arg: impl cmd::floor::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn now(self) -> Command {
        Command::new(TermType::Now)
    }

    pub fn time(self, arg: impl cmd::time::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn epoch_time(self, arg: impl cmd::epoch_time::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn iso8601(self, arg: impl cmd::iso8601::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn do_(self, arg: impl cmd::do_::Arg) -> Command    {
        arg.arg(None).into_cmd()
    }

    pub fn branch(self, arg: impl cmd::branch::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn range(self, arg: impl cmd::range::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn error(self, arg: impl cmd::error::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn expr(self, arg: impl cmd::expr::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn js(self, arg: impl cmd::js::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn info(self, arg: impl cmd::info::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn json(self, arg: impl cmd::json::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn http(self, arg: impl cmd::http::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn uuid(self, arg: impl cmd::uuid::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn circle(self, arg: impl cmd::circle::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn distance(self, arg: impl cmd::distance::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn geojson(self, arg: impl cmd::geojson::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn intersects(self, arg: impl cmd::intersects::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn line(self, arg: impl cmd::line::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn point(self, arg: impl cmd::point::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn polygon(self, arg: impl cmd::polygon::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn grant(self, arg: impl cmd::grant::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn wait(self, arg: impl cmd::wait::Arg) -> Command    {
        arg.arg().into_cmd()
    }

    pub fn asc(self, arg: impl cmd::asc::Arg) -> cmd::asc::Asc {
        cmd::asc::Asc(arg.arg().into_cmd())
    }

    pub fn desc(self, arg: impl cmd::desc::Arg) -> cmd::desc::Desc {
        cmd::desc::Desc(arg.arg().into_cmd())
    }

    pub fn index(self, arg: impl cmd::index::Arg) -> cmd::index::Index {
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
