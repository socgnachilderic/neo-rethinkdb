use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub use neor_macros::{func, Geometry};
use serde::{de::DeserializeOwned, Serialize};

use arguments::Permission;
use err::ReqlError;
use types::{Binary, DateTime, GeoJson};

pub use cmd::func::Func;
pub use command_tools::CommandArg;
pub use connection::*;
pub use proto::Command;

mod command_tools;
mod constants;
mod proto;

pub mod arguments;
pub mod cmd;
pub mod connection;
pub mod err;
pub mod types;

pub type Result<T> = std::result::Result<T, ReqlError>;

#[macro_export]
macro_rules! args {
    ( $($a:expr),* ) => {{ $crate::arguments::Args(($($a),*)) }};
}

#[doc(hidden)]
pub static VAR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[doc(hidden)]
pub fn var_counter() -> u64 {
    VAR_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[allow(non_camel_case_types)]
pub struct r;

impl r {
    /// Create a new connection to the database server.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.db_drop(db_name) → response
    /// ```
    ///
    /// Where:
    /// - db_name: &str | String | Cow<'static, str>
    /// - response: [DbResponse](crate::types::DbResponse)
    ///
    /// # Description
    ///
    /// If the connection cannot be established, a `ReqlDriverError` exception will be thrown.
    ///
    /// ## Examples
    ///
    /// Open a connection using the default host and port, specifying the default database.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().dbname("jam").connect().await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Open a new connection to the database.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection()
    ///         .dbname("jam")
    ///         .host("localhost")
    ///         .port(28015)
    ///         .connect()
    ///         .await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Open a new connection to the database,
    /// specifying a user/password combination for authentication.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection()
    ///         .dbname("jam")
    ///         .host("localhost")
    ///         .port(28015)
    ///         .user("jam_user", "jam_password")
    ///         .connect()
    ///         .await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [use_](crate::connection::Session::use_)
    /// - [close](crate::connection::Session::close)
    pub fn connection(&self) -> cmd::connect::ConnectionCommand {
        cmd::connect::ConnectionCommand::default()
    }

    /// Create a database.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.db_drop(db_name) → response
    /// ```
    ///
    /// Where:
    /// - db_name: `impl Into<String>`
    /// - response: [DbResponse](crate::types::DbResponse)
    ///
    /// # Description
    ///
    /// A RethinkDB database is a collection of tables, similar to relational databases.
    ///
    /// If a database with the same name already exists, the command throws `ReqlRuntimeError`.
    ///
    /// ## Note
    ///
    /// Only alphanumeric characters, hyphens and underscores are valid for the database name.
    ///
    /// ## Examples
    ///
    /// Create a database named ‘simbad’.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db_create("simbad")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response.dbs_created, Some(1));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [db_drop](Self::db_drop)
    /// - [db_list](Self::db_list)
    /// - [table_create](Self::table_create)
    pub fn db_create(&self, db_name: impl Into<String>) -> Command {
        cmd::db_create::new(db_name)
    }

    /// Drop a database.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.db_drop(db_name) → response
    /// ```
    ///
    /// Where:
    /// - db_name: `impl Into<String>`
    /// - response: [DbResponse](crate::types::DbResponse)
    ///
    /// # Description
    ///
    /// The database, all its tables, and corresponding data will be deleted.
    ///
    /// If the given database does not exist, the command throws `ReqlRuntimeError`.
    ///
    /// ## Examples
    ///
    /// Drop a database named ‘simbad’.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db_drop("simbad")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response.dbs_dropped, Some(1));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [db_create](Self::db_create)
    /// - [db_list](Self::db_list)
    /// - [table_create](Self::table_create)
    pub fn db_drop(&self, db_name: impl Into<String>) -> Command {
        cmd::db_drop::new(db_name)
    }

    /// List all database names in the system.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.db_list() → response
    /// ```
    ///
    /// Where:
    /// - response: Vec<String>
    ///
    /// ## Examples
    ///
    /// List all databases.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: Vec<String> = r.db_list().run(&conn).await?.unwrap().parse()?;
    ///
    ///     assert!(response.len() > 0);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [db_create](Self::db_create)
    /// - [db_drop](Self::db_drop)
    pub fn db_list(&self) -> Command {
        cmd::db_list::new()
    }

    /// Reference a database.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.db(db_name) → db
    /// ```
    ///
    /// Where:
    /// - db_name: `impl Into<String>`
    ///
    /// # Description
    ///
    /// The `db` command is optional. If it is not present in a query,
    /// the query will run against the database specified in the `db`
    /// argument given to [run](crate::Command::run) if one was specified.
    /// Otherwise, the query will run against the default database for the connection,
    /// specified in the `db` argument to [connection](Self::connection).
    ///
    /// ## Examples
    ///
    /// Explicitly specify a database for a query.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.db("heroes").table("simbad").run(&conn).await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [table](crate::Command::table)
    /// - [db_list](Self::db_list)
    pub fn db(&self, db_name: impl Into<String>) -> Command {
        cmd::db::new(db_name)
    }

    /// Create a table.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.table_create(table_name) → response
    /// db.table_create(args!(table_name, options)) → response
    /// ```
    ///
    /// Where:
    /// - table_name: `impl Into<String>` | [Command](crate::Command)
    /// - options: [TableCreateOption](crate::arguments::TableCreateOption)
    /// - response: [DbResponse](crate::types::DbResponse)
    ///
    /// # Description
    ///
    /// A RethinkDB table is a collection of JSON documents.
    ///
    /// If a table with the same name already exists,
    /// the command throws `ReqlOpFailedError`.
    ///
    /// ```text
    /// Note: Only alphanumeric characters and underscores are valid for the table name.
    ///
    /// Invoking table_create without specifying a database using db creates a table in
    /// the database specified in connect, or test if no database was specified.
    /// ```
    ///
    /// The [data type](https://rethinkdb.com/docs/data-types/) of a primary key is usually a string
    /// (like a UUID) or a number, but it can also be a time, binary object, boolean or an array.
    /// Data types can be mixed in the primary key field, but all values must be unique. Using an array
    /// as a primary key causes the primary key to behave like a compound index; read the documentation on
    /// [compound secondary indexes](https://rethinkdb.com/docs/secondary-indexes/python/#compound-indexes)
    /// for more information, as it applies to primary keys as well.
    /// (Note that the primary index still only covers a single field,
    /// while compound secondary indexes can cover multiple fields in a single index.)
    /// Primary keys cannot be objects.
    ///
    /// Tables will be available for writing when the command returns.
    ///
    /// ## Examples
    ///
    /// Create a table named ‘simbad’ with the default settings.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db("test")
    ///         .table_create("simbad")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.tables_created > Some(0));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Create a table named ‘simbad’ using the field ‘name’ as primary key.
    ///
    /// ```
    /// use neor::arguments::TableCreateOption;
    /// use neor::types::DbResponse;
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let table_create_option = TableCreateOption::default().primary_key("name");
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db("test")
    ///         .table_create(args!("simbad", table_create_option))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.tables_created > Some(0));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Create a table set up for two shards and three replicas per shard.
    /// This requires three available servers.
    ///
    /// ```
    /// use neor::arguments::{Replicas, TableCreateOption};
    /// use neor::types::DbResponse;
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let table_create_option = TableCreateOption::default()
    ///         .shards(2)
    ///         .replicas(Replicas::Int(3));
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db("test")
    ///         .table_create(args!("simbad", table_create_option))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.tables_created > Some(0));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Read [Sharding and replication](https://rethinkdb.com/docs/sharding-and-replication/)
    /// for a complete discussion of the subject, including advanced topics.
    ///
    /// # Related commands
    /// - [table_drop](Self::table_drop)
    /// - [table_list](Self::table_list)
    pub fn table_create(&self, args: impl cmd::table_create::TableCreateArg) -> Command {
        cmd::table_create::new(args)
    }

    /// Drop a table.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.table_drop(table_name) → response
    /// ```
    ///
    /// Where:
    /// - table_name: `impl Into<String>` | [Command](crate::Command)
    /// - response: [DbResponse](crate::types::DbResponse)
    ///
    /// # Description
    ///
    /// The table and all its data will be deleted.
    ///
    /// If the given table does not exist in the database,
    /// the command throws `ReqlRuntimeError`.
    ///
    /// ## Examples
    ///
    /// Drop a table named ‘simbad’.
    ///
    /// ```
    /// use neor::types::DbResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: DbResponse = r.db("test")
    ///         .table_drop("simbad")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.tables_dropped > Some(0));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [table_create](Self::table_create)
    /// - [table_list](Self::table_list)
    pub fn table_drop(&self, table_name: impl Into<CommandArg>) -> Command {
        cmd::table_drop::new(table_name)
    }

    /// List all table names in a database.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.table_list() → response
    /// ```
    ///
    /// Where:
    /// - response: Vec<String>
    ///
    /// ## Examples
    ///
    /// List all tables of the ‘test’ database.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: Vec<String> = r.db("test")
    ///         .table_list()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.len() > 0);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [table_create](Self::table_create)
    /// - [table_drop](Self::table_drop)
    pub fn table_list(&self) -> Command {
        cmd::table_list::new()
    }

    /// Return all documents in a table.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// db.table(name) → table
    /// db.table(args!(name, options)) → table
    /// r.table(name) → table
    /// r.table(args!(name, options)) → table
    /// ```
    ///
    /// Where:
    /// - name: `impl Into<String>` | [Command](crate::Command)
    /// - options: [TableOption](crate::arguments::TableOption)
    ///
    /// # Description
    ///
    /// Other commands may be chained after `table` to return a subset of documents
    /// (such as [get](crate::Command::get) and [filter](crate::Command::filter))
    /// or perform further processing.
    ///
    /// ## Examples
    ///
    /// Return all documents in the table ‘simbad’ of the default database.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad").run(&conn).await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return all documents in the table ‘simbad’ of the database ‘heroes’.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.db("heroes").table("simbad").run(&conn).await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Allow potentially out-of-date data in exchange for faster reads.
    ///
    /// ```
    /// use neor::arguments::{ReadMode, TableOption};
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let table_option = TableOption::default().read_mode(ReadMode::Outdated);
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.db("heroes").table(args!("simbad", table_option)).run(&conn).await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [filter](crate::Command::filter)
    /// - [get](crate::Command::get)
    pub fn table(&self, args: impl cmd::table::TableArg) -> Command {
        cmd::table::new(args)
    }

    /// Transform each element of one or more sequences
    /// by applying a mapping function to them.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.map(func) → stream
    /// sequence.map(sequence, func) → stream
    /// sequence.map(sequences, func) → stream
    /// r.map(sequence, func) → stream
    /// r.map(sequence, sequence, func) → stream
    /// r.map(sequence, sequences, func) → stream
    /// ```
    ///
    /// Where:
    /// - func: func!(...)
    /// - sequence: [Command](crate::Command)
    /// - sequences: [...] | &[...] | vec![...]
    ///
    /// # Description
    ///
    /// If `map` is run with two or more sequences, it will iterate
    /// for as many items as there are in the shortest sequence.
    ///
    /// Note that `map` can only be applied to sequences, not single values.
    /// If you wish to apply a function to a single value/selection (including an array),
    /// use the [do_](Self::do_) command.
    ///
    /// ## Examples
    ///
    /// Return the first five squares.
    ///
    /// ```
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: Vec<u8> = r.expr([1, 2, 3, 4, 5])
    ///         .map(func!(|val| val.clone() * val))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response, [1, 4, 9, 16, 25]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Sum the elements of three sequences.
    ///
    /// ```
    /// use neor::{args, func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let sequence1 = r.expr([100, 200, 300, 400]);
    ///     let sequence2 = r.expr([10, 20, 30, 40]);
    ///     let sequence3 = r.expr([1, 2, 3, 4]);
    ///
    ///     let conn = r.connection().connect().await?;
    ///     let response: Vec<u32> = r.map(sequence1, args!(
    ///             [sequence2, sequence3],
    ///             func!(|val1, val2, val3| val1 + val2 + val3)
    ///         ))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert_eq!(response, [111, 222, 333, 444]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Rename a field when retrieving documents
    /// using `map` and [merge](crate::Command::merge).
    ///
    /// This example renames the field `id` to `user_id`
    /// when retrieving documents from the table `users`.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .map(func!(|doc| {
    ///             let mut user = HashMap::new();
    ///             user.insert("user_id", doc.g("id"));
    ///             
    ///             doc.merge(r.hash_map(user)).without("id")
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Assign every superhero an archenemy.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use neor::{args, func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("heroes")
    ///         .map(args!(r.table("villains"), func!(|hero, villain| {
    ///             let mut villain_obj = HashMap::new();
    ///             villain_obj.insert("villain", villain);
    ///
    ///             hero.merge(r.hash_map(villain_obj))
    ///         })))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [concat_map](crate::Command::concat_map)
    /// - [reduce](Self::reduce)
    /// - [do_](Self::do_)
    pub fn map(&self, sequence: Command, args: impl cmd::map::MapArg) -> Command {
        sequence.map(args)
    }

    /// Merge two or more sequences.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// stream.union(sequence) → stream
    /// stream.union(sequences) → stream
    /// stream.union(args!(sequence, options)) → stream
    /// stream.union(args!(sequences, options)) → stream
    /// ```
    ///
    /// Where:
    /// - sequence: [Command](crate::Command)
    /// - sequences: `impl IntoIterator<Command>`
    /// - options: [UnionOption](crate::arguments::UnionOption)
    ///
    /// ## Examples
    ///
    /// Construct a stream of all characters.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .union(r.table("kirikou"))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn union(&self, stream: Command, args: impl cmd::union::UnionArg) -> Command {
        stream.union(args)
    }

    /// Takes a stream and partitions it into multiple
    /// groups based on the fields or functions provided.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.group(field) → grouped_stream
    /// sequence.group(func) → grouped_stream
    /// sequence.group(args!(field, options)) → grouped_stream
    /// sequence.group(args!(func, options)) → grouped_stream
    /// r.group(sequence, field) → grouped_stream
    /// r.group(sequence, func) → grouped_stream
    /// r.group(sequence, args!(field, options)) → grouped_stream
    /// r.group(sequence, args!(func, options)) → grouped_stream
    /// ```
    ///
    /// Where:
    /// - field: &str | [&str; N]
    /// - func: func!(...) | [func!(...); N]
    /// - grouped_stream: [GroupedStream](crate::types::GroupedStream)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// With the `multi` flag single documents can be assigned to multiple groups,
    /// similar to the behavior of
    /// [multi-indexes](https://rethinkdb.com/docs/secondary-indexes/javascript).
    /// When `multi` is `true` and the grouping value is an array, documents
    /// will be placed in each group that corresponds to the elements of the array.
    /// If the array is empty the row will be ignored.
    ///
    /// Suppose that the table games has the following data:
    ///
    /// ```text
    /// [
    ///     {id: 2, player: "Moussa", points: 15, class: "ranked"},
    ///     {id: 5, player: "Fatou", points: 7, class: "free"},
    ///     {id: 11, player: "Moussa", points: 10, class: "free"},
    ///     {id: 12, player: "Fatou", points: 2, class: "free"}
    /// ]
    /// ```
    ///
    /// ## Examples
    ///
    /// Group games by player.
    ///
    /// ```
    /// use neor::types::{GroupedItem, GroupedStream};
    /// use neor::{args, r, Converter, Result};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Eq)]
    /// struct Player {
    ///     id: u8,
    ///     player: String,
    ///     points: u8,
    ///     class: String,
    /// }
    ///
    /// impl Player {
    ///     fn new(id: u8, player: &str, points: u8, class: &str) -> Self {
    ///         Self {
    ///             id,
    ///             points,
    ///             player: player.to_owned(),
    ///             class: class.to_owned(),
    ///         }
    ///     }
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let expected_data = vec![
    ///         GroupedItem {
    ///             group: String::from("Fatou"),
    ///             values: vec![
    ///                 Player::new(5, "Fatou", 7, "free"),
    ///                 Player::new(12, "Fatou", 2, "free"),
    ///             ]
    ///         },
    ///         GroupedItem {
    ///             group: String::from("Moussa"),
    ///             values: vec![
    ///                 Player::new(2, "Moussa", 15, "ranked"),
    ///                 Player::new(11, "Moussa", 10, "free"),
    ///             ]
    ///         },
    ///     ];
    ///     let conn = r.connection().connect().await?;
    ///     let response: GroupedStream<String, Player> = r.table("games")
    ///         .group("player")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.collect() == expected_data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ungroup](crate::Command::ungroup)
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    pub fn group(&self, sequence: Command, args: impl cmd::group::GroupArg) -> Command {
        sequence.group(args)
    }

    /// Produce a single value from a sequence through
    /// repeated application of a reduction function.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.fold(base, func) → value
    /// ```
    ///
    /// Where:
    /// - base: `impl Serialize` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The reduction function can be called on:
    ///
    /// - two elements of the sequence
    /// - one element of the sequence and one result of a previous reduction
    /// - two results of previous reductions
    ///
    /// The reduction function can be called on the results of
    /// two previous reductions because the `reduce` command is
    /// distributed and parallelized across shards and CPU cores.
    /// A common mistaken when using the `reduce` command is to
    /// suppose that the reduction is executed from left to right.
    /// [Read the map-reduce in RethinkDB](https://rethinkdb.com/docs/map-reduce/)
    /// article to see an example.
    ///
    /// If the sequence is empty, the server will produce a
    /// `ReqlRuntimeError` that can be caught with default.
    /// If the sequence has only one element, the first element will be returned.
    ///
    /// ## Examples
    ///
    /// Return the number of documents in the table posts.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("posts")
    ///         .map(func!(|| r.expr(1)))
    ///         .reduce(func!(|left, right| left + right))
    ///         .default(0)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A shorter way to execute this query is to use [count](Self::count).
    ///
    /// ## Examples
    ///
    /// Suppose that each `post` has a field `comments` that is an array of comments.
    /// Return the maximum number comments per post.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("posts")
    ///         .map(func!(|post| post.g("comments").count(())))
    ///         .reduce(func!(|left, right| r.branch(
    ///             left.gt(&right),
    ///             args!(left, right)
    ///         )))
    ///         .default(0)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A shorter way to execute this query is to use [max](Self::max).
    ///
    /// # Related commands
    /// - [group](crate::Command::group)
    /// - [map](Self::map)
    /// - [concat_map](crate::Command::concat_map)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    pub fn reduce(&self, sequence: Command, func: Func) -> Command {
        sequence.reduce(func)
    }

    /// Count the number of elements in sequence or key/value pairs in an object,
    /// or returns the size of a string or binary object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// binary.count(()) → number
    /// string.count(()) → number
    /// object.count(()) → number
    /// sequence.count(()) → number
    /// sequence.count(args!(value)) → number
    /// sequence.count(func) → number
    /// r.count(query_cmd) → number
    /// r.count(query_cmd, args!(value)) → number
    /// r.count(query_cmd, func) → number
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize`
    /// - func: [Func](crate::Func)
    /// - sequence, binary, string, object, query_cmd: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// When `count` is called on a sequence with a predicate value or function,
    /// it returns the number of elements in the sequence equal to that value or
    /// where the function returns `true`. On a [binary](Self::binary) object, `count`
    /// returns the size of the object in bytes; on strings, `count` returns the string’s length.
    /// This is determined by counting the number of Unicode codepoints in the string,
    /// counting combining codepoints separately.
    ///
    /// ## Examples
    ///
    /// Count the number of users.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .count(())
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Count the number of 18 year old users.
    ///
    /// ```
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .g("age")
    ///         .count(args!(18))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Count the number of users over 18.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .g("age")
    ///         .count(func!(|age| age.gt(18)))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the length of a Unicode string.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: usize = r.expr("こんにちは")
    ///         .count(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 5);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the length of an array.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: usize = r.expr(['0','1','2'])
    ///         .count(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](crate::Command::group)
    pub fn count(&self, query: Command, args: impl cmd::count::CountArg) -> Command {
        query.count(args)
    }

    /// Sum all the elements of sequence.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.sum(()) → number
    /// sequence.sum(field) → number
    /// sequence.sum(func) → number
    /// r.sum(sequence) → number
    /// r.sum(sequence, field) → number
    /// r.sum(sequence, func) → number
    /// ```
    ///
    /// Where:
    /// - field: `&str` | `String` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// If called with a field name, sums all the values of that field in
    /// the sequence, skipping elements of the sequence that lack that field.
    /// If called with a function, calls that function on every element of the
    /// sequence and sums the results, skipping elements of the sequence
    /// where that function returns `None` or non-existence error.
    ///
    /// Returns `0` when called on an empty sequence.
    ///
    /// ## Examples
    ///
    /// What's 3 + 5 + 7?
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr([3, 5, 7])
    ///         .sum(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 15);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// How many points have been scored across all games?
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("games")
    ///         .sum("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// How many points have been scored across all games, counting bonus points?
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("games")
    ///         .sum(func!(|game| game.g("points") + game.g("bonus_points")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](crate::Command::group)
    pub fn sum(&self, sequence: Command, args: impl cmd::sum::SumArg) -> Command {
        sequence.sum(args)
    }

    /// Averages all the elements of sequence.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.avg(()) → Option<f64>
    /// sequence.avg(field) → Option<f64>
    /// sequence.avg(func) → Option<f64>
    /// r.avg(sequence) → Option<f64>
    /// r.avg(sequence, field) → Option<f64>
    /// r.avg(sequence, func) → Option<f64>
    /// ```
    ///
    /// Where:
    /// - field: `&str` | `String` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// If called with a field name, averages all the values of that field in
    /// the sequence, skipping elements of the sequence that lack that field.
    /// If called with a function, calls that function on every element of the
    /// sequence and averages the results, skipping elements of the sequence
    /// where that function returns `None` or non-existence error.
    ///
    /// Produces a non-existence error when called on an empty sequence.
    /// You can handle this case with `default`.
    ///
    /// ## Examples
    ///
    /// What's the average of 3, 5 and 7?
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: Option<f64> = r.expr([3, 5, 7])
    ///         .avg(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == Some(5.));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// What's the average number of points scored in a games?
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("games")
    ///         .avg("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// What's the average number of points scored in a games, counting bonus points?
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("games")
    ///         .avg(func!(|game| game.g("points") + game.g("bonus_points")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](crate::Command::group)
    pub fn avg(&self, sequence: Command, args: impl cmd::avg::AvgArg) -> Command {
        sequence.avg(args)
    }

    /// Finds the minimum element of a sequence.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.min(()) → element
    /// sequence.min(field) → element
    /// sequence.min(func) → element
    /// sequence.min(options) → element
    /// r.min(sequence) → element
    /// r.min(sequence, field) → element
    /// r.min(sequence, func) → element
    /// r.min(sequence, options) → element
    /// ```
    ///
    /// Where:
    /// - field: `&str` | `String` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    /// - options: [MinOption](crate::arguments::MinOption)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `min` command can be called with:
    /// - a `field name`, to return the element of the sequence
    /// with the largest value in that field;
    /// - a `function`, to apply the function to every element within the sequence
    /// and return the element which returns the largest value from the function,
    /// ignoring any elements where the function produces a non-existence error;
    /// - an `index` (the primary key or a secondary index), to return the element
    /// of the sequence with the largest value in that index;
    ///
    /// For more information on RethinkDB’s sorting order, read the section in
    /// [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order).
    ///
    /// Calling `min` on an empty sequence will throw a non-existence error;
    /// this can be handled using the [default](crate::Command::default) command.
    ///
    /// ## Examples
    ///
    /// Return the minimum value in the list [3, 5, 7].
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr([3, 5, 7])
    ///         .min(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the user who has scored the fewest points.
    ///
    /// ```
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .min("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// The same as above, but using a secondary index on the `points` field.
    ///
    /// ```
    /// use neor::arguments::MinOption;
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .min(MinOption::default().index("points"))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the user who has scored the fewest points,
    /// adding in bonus points from a separate field using a function.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .min(func!(|user| user.g("points") + user.g("bonus_points")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the highest number of points any user has ever scored.
    /// This returns the value of that `points` field, not a document.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.table("users")
    ///         .min("points")
    ///         .g("points")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [max](Self::max)
    /// - [group](crate::Command::group)
    pub fn min(&self, sequence: Command, args: impl cmd::min::MinArg) -> Command {
        sequence.min(args)
    }

    /// Finds the maximum element of a sequence.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.max(()) → element
    /// sequence.max(field) → element
    /// sequence.max(func) → element
    /// sequence.max(options) → element
    /// r.max(sequence) → element
    /// r.max(sequence, field) → element
    /// r.max(sequence, func) → element
    /// r.max(sequence, options) → element
    /// ```
    ///
    /// Where:
    /// - field: &str, String, Cow<'static, str>
    /// - func: func!(...)
    /// - options: [MaxOption](crate::arguments::MaxOption)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `max` command can be called with:
    /// - a `field name`, to return the element of the sequence
    /// with the largest value in that field;
    /// - a `function`, to apply the function to every element within the sequence
    /// and return the element which returns the largest value from the function,
    /// ignoring any elements where the function produces a non-existence error;
    /// - an `index` (the primary key or a secondary index), to return the element
    /// of the sequence with the largest value in that index;
    ///
    /// For more information on RethinkDB’s sorting order, read the section in
    /// [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order).
    ///
    /// Calling `max` on an empty sequence will throw a non-existence error;
    /// this can be handled using the [default](crate::Command::default) command.
    ///
    /// ## Examples
    ///
    /// Return the maximum value in the list [3, 5, 7].
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr([3, 5, 7])
    ///         .max(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 7);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the user who has scored the most points.
    ///
    /// ```
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .max("points")
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// The same as above, but using a secondary index on the `points` field.
    ///
    /// ```
    /// use neor::arguments::MaxOption;
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .max(MaxOption::default().index("points"))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the user who has scored the most points,
    /// adding in bonus points from a separate field using a function.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("users")
    ///         .max(func!(|user| user.g("points") + user.g("bonus_points")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the highest number of points any user has ever scored.
    /// This returns the value of that `points` field, not a document.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.table("users")
    ///         .max("points")
    ///         .g("points")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 15);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [group](crate::Command::group)
    pub fn max(&self, sequence: Command, args: impl cmd::max::MaxArg) -> Command {
        sequence.max(args)
    }

    /// Removes duplicate elements from a sequence.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// sequence.distinct(()) → array
    /// table.distinct(options) → stream
    /// r.distinct(sequence) → array
    /// r.distinct(table, options) → stream
    /// ```
    ///
    /// Where:
    /// - options: [DistinctOption](crate::arguments::DistinctOption)
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `distinct` command can be called on any sequence or table with an index.
    ///
    /// ```text
    /// While `distinct` can be called on a table without an index,
    /// the only effect will be to convert the table into a stream;
    /// the content of the stream will not be affected.
    /// ```
    ///
    /// ## Examples
    ///
    /// Which unique villains have been vanquished by Marvel heroes?
    ///
    /// ```
    /// use neor::{func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("marvel")
    ///         .concat_map(func!(|hero| hero.g("villain_list")))
    ///         .distinct(())
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Topics in a table of messages have a secondary index on them,
    /// and more than one message can have the same topic.
    /// What are the unique topics in the table?
    ///
    /// ```
    /// use neor::arguments::DistinctOption;
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("messages")
    ///         .distinct(DistinctOption::default().index("topics"))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The above structure is functionally identical to:
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("messages")
    ///         .g("topics")
    ///         .distinct(())
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// However, the first form (passing the index as an argument to `distinct`) is faster,
    /// and won’t run into array limit issues since it’s returning a stream.
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [concat_map](crate::Command::concat_map)
    /// - [group](crate::Command::group)
    pub fn distinct(
        &self,
        seq_or_table: Command,
        args: impl cmd::distinct::DistinctArg,
    ) -> Command {
        seq_or_table.distinct(args)
    }

    /// When called with values, returns `true`
    /// if a sequence contains all the specified values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// query.contains(value) → bool
    /// query.contains(args!(values)) → bool
    /// r.contains(sequence, value) → bool
    /// r.contains(sequence, args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command) | [Func](crate::Func)
    /// - values: `impl IntoIterator<Item = T>` | `impl IntoIterator<Item = Command>`
    /// - sequence: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// When called with predicate functions, returns `true`
    /// if for each predicate there exists at least one element
    /// of the stream where that predicate returns `true`.
    ///
    /// Values and predicates may be mixed freely in the argument list.
    ///
    /// ## Examples
    ///
    /// Has Iron Man ever fought Superman?
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("marvel")
    ///         .get("ironman")
    ///         .g("opponents")
    ///         .contains("superman")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Has Iron Man ever defeated Superman in battle?
    ///
    /// ```
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("marvel")
    ///         .get("ironman")
    ///         .g("battles")
    ///         .contains(func!(|battle| battle.g("winner").eq("ironman").and(
    ///             battle.g("loser").eq("superman")
    ///         )))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return all heroes who have fought both Loki and the Hulk.
    ///
    /// ```
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("marvel")
    ///         .filter(func!(|hero| hero.g("opponents").contains(["loki", "hulk"])))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Use contains with a predicate function to simulate an or.
    /// Return the Marvel superheroes who live in Detroit, Chicago or Hoboken.
    ///
    /// ```
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("marvel")
    ///         .filter(func!(|hero| r.expr(["Detroit", "Chicago", "Hoboken"]).contains(hero.g("city"))))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [concat_map](crate::Command::concat_map)
    /// - [group](crate::Command::group)
    pub fn contains(&self, sequence: Command, value: impl Into<CommandArg>) -> Command {
        sequence.contains(value)
    }

    /// TODO Write docs
    #[doc(hidden)]
    pub fn literal(&self, value: impl Into<CommandArg>) -> Command {
        cmd::literal::new(value)
    }

    /// Creates an object from a list of key-value pairs,
    /// where the keys must be strings.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.object(values) → object
    /// ```
    ///
    /// Where:
    /// - values: `impl IntoIterator<Item = value>`
    /// - value: `impl Serialize` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Create a simple object.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    /// struct Post {
    ///     id: String,
    ///     title: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let post = Post {
    ///         id: "id1".to_string(),
    ///         title: "title1".to_string(),
    ///     };
    ///     let response: Post = r.object(["id", "id1", "title", "title1"])
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == post);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [coerce_to](crate::Command::coerce_to)
    /// - [merge](crate::Command::merge)
    /// - [keys](crate::Command::keys)
    pub fn object<S, T>(&self, values: T) -> Command
    where
        S: Into<CommandArg>,
        T: IntoIterator<Item = S>,
    {
        cmd::object::new(values)
    }

    /// Compute the logical “and” of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.and(value) → bool
    /// cmd_value.and(args!(values)) → bool
    /// r.and(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `bool` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = bool>` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `and` command can be used as an infix operator after its
    /// first argument (`r.expr(true).and(false)`) or given all of
    /// its arguments as parameters (`r.and(args!([true, false]))`).
    ///
    /// Calling `or` with zero arguments will return `false`.
    ///
    /// ## Examples
    ///
    /// Return whether either true or false evaluate to true.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.expr(true)
    ///         .and(false)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return whether any of true, true or true evaluate to true.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.and(args!([true, true, true]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [or](Self::or)
    pub fn and(&self, args: impl cmd::and::AndArg) -> Command {
        cmd::and::new(args)
    }

    /// Compute the logical “or” of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.or(value) → bool
    /// cmd_value.or(args!(values)) → bool
    /// r.or(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `bool` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = bool>` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `or` command can be used as an infix operator after
    /// its first argument (`r.expr(true).or(false)`) or given all
    /// of its arguments as parameters (`r.or(args!([true, false]))`).
    ///
    /// Calling `or` with zero arguments will return `false`.
    ///
    /// ## Examples
    ///
    /// Return whether either true or false evaluate to true.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.expr(true)
    ///         .or(false)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return whether any of false, false or false evaluate to false.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.or(args!([false, false, false]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Note
    ///
    /// When using `or` inside a `filter` predicate to test the values of
    /// fields that may not exist on the documents being tested,
    /// you should use the `default` command with those fields so
    /// they explicitly return `false`.
    ///
    /// ```
    /// use neor::{func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("table")
    ///         .filter(func!(|post| post.g("category")
    ///             .default("foo")
    ///             .eq("article")
    ///             .or(post.g("genre").default("foo").eq("mystery"))
    ///         ))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [and](Self::and)
    pub fn or(&self, args: impl cmd::or::OrArg) -> Command {
        cmd::or::new(args)
    }

    /// Test if two or more values are equal.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.eq(value) → bool
    /// cmd_value.eq(args!(values)) → bool
    /// r.eq(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// See if a user’s `role` field is set to `administrator`.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("users")
    ///         .get(1)
    ///         .g("role")
    ///         .eq("administrator")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// See if three variables contain equal values.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.eq(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ne](Self::ne)
    /// - [and](Self::and)
    /// - [or](Self::or)
    pub fn eq(&self, args: impl cmd::eq::EqArg) -> Command {
        cmd::eq::new(args)
    }

    /// Test if two or more values are not equal.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.ne(value) → bool
    /// cmd_value.ne(args!(values)) → bool
    /// r.ne(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// See if a user’s `role` field is not set to `administrator`.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("users")
    ///         .get(1)
    ///         .g("role")
    ///         .ne("administrator")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// See if three variables do not contain equal values.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.ne(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [and](Self::and)
    /// - [or](Self::or)
    pub fn ne(&self, args: impl cmd::ne::NeArg) -> Command {
        cmd::ne::new(args)
    }

    /// Compare values, testing if the left-hand value is greater than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.gt(value) → bool
    /// cmd_value.gt(args!(values)) → bool
    /// r.gt(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// Test if a player has scored more than 10 points.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .gt(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from lowest to highest,
    /// with no values being equal to one another.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.gt(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    pub fn gt(&self, args: impl cmd::gt::GtArg) -> Command {
        cmd::gt::new(args)
    }

    /// Compare values, testing if the left-hand value is greater than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.ge(value) → bool
    /// cmd_value.ge(args!(values)) → bool
    /// r.ge(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// Test if a player has scored more than 10 points.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .ge(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from lowest to highest,
    /// with no values being equal to one another.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.ge(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    pub fn ge(&self, args: impl cmd::ge::GeArg) -> Command {
        cmd::ge::new(args)
    }

    /// Compare values, testing if the left-hand value is less than the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.lt(value) → bool
    /// cmd_value.lt(args!(values)) → bool
    /// r.lt(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// Test if a player has scored less than 10 points.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .lt(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from highest to lowest,
    /// with no values being equal to one another.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.lt(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [le](Self::le)
    pub fn lt(&self, args: impl cmd::lt::LtArg) -> Command {
        cmd::lt::new(args)
    }

    /// Compare values, testing if the left-hand value is
    /// less than or equal to the right-hand.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_value.le(value) → bool
    /// cmd_value.le(args!(values)) → bool
    /// r.le(args!(values)) → bool
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    /// - values: `impl IntoIterator<Item = T>`
    ///
    /// ## Examples
    ///
    /// Test if a player has scored 10 points or less.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.table("players")
    ///         .get(1)
    ///         .g("score")
    ///         .le(10)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test if variables are ordered from highest to lowest.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.le(args!([20, 10, 15]))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == true);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    pub fn le(&self, args: impl cmd::le::LeArg) -> Command {
        cmd::le::new(args)
    }

    /// Compute the logical inverse (not) of an expression.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// !cmd_bool
    /// cmd_bool.not() → bool
    /// r.not(cmd_bool) → bool
    /// ```
    ///
    /// Where:
    /// - cmd_bool: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// `not` can be called either via method chaining, immediately after
    /// an expression that evaluates as a boolean value, or by passing
    /// the expression as a parameter to `not`. All values that are not
    /// `false` or `None` will be converted to `true`.
    ///
    /// ## Examples
    ///
    /// Not true is false.
    ///
    /// ```
    /// use std::ops::Not;
    ///
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: bool = r.expr(true)
    ///         .not()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: bool = r.not(r.expr(true))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: bool = (!r.expr(true))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == false &&
    ///         response == response2 &&
    ///         response == response3
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    pub fn not(&self, cmd_bool: Command) -> Command {
        !cmd_bool
    }

    /// Generate a random number between given (or implied) bounds.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.random(()) → number
    /// r.random(param_number) → number
    /// r.random(args!(param_number, param_number)) → number
    /// r.random(args!(param_number, param_number, options)) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize, f32, f64` | [Command](crate::Command)
    /// - options: [RandomOption](crate::arguments::RandomOption)
    ///
    /// # Description
    ///
    /// `random` takes zero, one or two arguments.
    ///
    /// - With *zero* arguments, the result will be a floating-point
    /// number in the range `[0,1)` (from 0 up to but not including 1).
    /// - With *one* argument x, the result will be in the range `[0,x)`, and will
    /// be integer unless `RandomOption::default().float(true)` is given as an option.
    /// Specifying a floating point number without the float option will raise an error.
    /// - With *two* arguments x and y, the result will be in the range
    /// `[x,y)`, and will be integer unless `RandomOption::default().float(true)` is given
    /// as an option. If x and y are equal an error will occur, unless the floating-point
    /// option has been specified, in which case x will be returned.
    /// Specifying a floating point number without the float option will raise an error.
    ///
    /// ## Note
    ///
    /// The last argument given will always be the ‘open’ side of the range, but when
    /// generating a floating-point number, the ‘open’ side may be less than the ‘closed’ side.
    ///
    /// ## Examples
    ///
    /// Generate a random number in the range `[0,1)`
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.random(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a random integer in the range `[0,100)`
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.random(100.)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 3);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a random number in the range `(-2.24,1.59]`
    ///
    /// ```
    /// use neor::arguments::RandomOption;
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: f64 = r.random(args!(
    ///             1.59, -2.24,
    ///             RandomOption::default().float(true)
    ///         ))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 0.);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [sample](crate::Command::sample)
    pub fn random(&self, args: impl cmd::random::RandomArg) -> Command {
        cmd::random::new(args)
    }

    /// Rounds the given value to the nearest whole integer.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.round(param_number) → number
    /// number.round() → number
    /// ```
    ///
    /// Where:
    /// - param_number: `f32` | `f64` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// For example, values of 1.0 up to but not including 1.5
    /// will return 1.0, similar to [floor](Self::floor); values
    /// of 1.5 up to 2.0 will return 2.0, similar to [ceil](Self::ceil).
    ///
    /// ## Examples
    ///
    /// Round 12.345 to the nearest integer.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.round(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 12);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `round` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Round -12.345 to the nearest integer.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .round()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -12);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [round](Self::round)
    pub fn round(&self, args: impl cmd::round::RoundArg) -> Command {
        cmd::round::new(args)
    }

    /// Rounds the given value up, returning the smallest integer value
    /// greater than or equal to the given value (the value’s ceiling).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.ceil(param_number) → number
    /// number.ceil() → number
    /// ```
    ///
    /// Where:
    /// - param_number: `f32` | `f64` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Return the ceiling of 12.345.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.ceil(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `ceil` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Return the ceiling of -12.345.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .ceil()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [floor](Self::floor)
    /// - [round](Self::round)
    pub fn ceil(&self, args: impl cmd::ceil::CeilArg) -> Command {
        cmd::ceil::new(args)
    }

    /// Rounds the given value down, returning the largest integer
    /// value less than or equal to the given value (the value’s floor).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.floor(param_number) → number
    /// number.floor() → number
    /// ```
    ///
    /// Where:
    /// - param_number: `f32` | `f64` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Return the floor of 12.345.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.floor(12.345)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The `floor` command can also be chained after an expression.
    ///
    /// ## Examples
    ///
    /// Return the floor of -12.345.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(-12.345)
    ///         .floor()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -13);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [round](Self::round)
    pub fn floor(&self, args: impl cmd::floor::FloorArg) -> Command {
        cmd::floor::new(args)
    }

    /// Compute the arithmetic "and" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number & cmd_number
    /// number.bit_and(param_number) → number
    /// r.bit_and(param_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise AND is a binary operation that takes two equal-length binary
    /// representations and performs the logical AND operation on each pair of
    /// the corresponding bits, which is equivalent to multiplying them.
    /// Thus, if both bits in the compared position are 1,
    /// the bit in the resulting binary representation is 1 (1 × 1 = 1);
    /// otherwise, the result is 0 (1 × 0 = 0 and 0 × 0 = 0).
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "and" of 5 and 3
    ///
    /// ```
    /// use std::ops::BitAnd;
    ///
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(5)
    ///         .bit_and(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_and(r.expr(5), 3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(5) & r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(5)
    ///         .bitand(r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 1 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_or](Self::bit_or)
    /// - [bit_not](Self::bit_not)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_and<S, T>(&self, number1: S, number2: T) -> Command
    where
        S: Into<CommandArg>,
        T: Into<CommandArg>,
    {
        number1.into().to_cmd().bit_and(number2)
    }

    /// Compute the arithmetic "or" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number | cmd_number
    /// number.bit_or(param_number) → number
    /// r.bit_or(param_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise OR is a binary operation that takes two bit patterns
    /// of equal length and performs the logical inclusive OR operation
    /// on each pair of corresponding bits. The result in each position
    /// is 0 if both bits are 0, while otherwise the result is 1.
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "or" of 6 and 4
    ///
    /// ```
    /// use std::ops::BitOr;
    ///
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(5)
    ///         .bit_or(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_xor(r.expr(5), 3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(5) ^ r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(5)
    ///         .bitor(r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 7 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_or<S, T>(&self, number1: S, number2: T) -> Command
    where
        S: Into<CommandArg>,
        T: Into<CommandArg>,
    {
        number1.into().to_cmd().bit_or(number2)
    }

    /// Compute the arithmetic "and" of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// cmd_number ^ cmd_number
    /// number.bit_xor(param_number) → number
    /// r.bit_xor(param_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    /// - cmd_number: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise XOR is a binary operation that takes two bit patterns
    /// of equal length and performs the logical exclusive OR operation
    /// on each pair of corresponding bits. The result in each position
    /// is 1 if only the first bit is 1 or only the second bit is 1,
    /// but will be 0 if both are 0 or both are 1.
    /// In this we perform the comparison of two bits, being 1 if the
    /// two bits are different, and 0 if they are the same.
    ///
    /// ## Examples
    ///
    /// Compute the arithmetic "and" of 6 and 4
    ///
    /// ```
    /// use std::ops::BitXor;
    ///
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(6)
    ///         .bit_xor(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_xor(r.expr(6), 4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response3: i32 = (r.expr(6) ^ r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response4: i32 = r.expr(6)
    ///         .bitxor(r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(
    ///         response == 2 &&
    ///         response == response2 &&
    ///         response == response3 &&
    ///         response == response4
    ///     );
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    pub fn bit_xor<S, T>(&self, number1: S, number2: T) -> Command
    where
        S: Into<CommandArg>,
        T: Into<CommandArg>,
    {
        number1.into().to_cmd().bit_xor(number2)
    }

    /// Compute the arithmetic inverse (not) of an expression.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_not() → number
    /// r.bit_not(param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// A bitwise NOT, or complement, is a unary operation that performs logical
    /// negation on each bit, forming the ones’ complement of the given binary value.
    /// Bits that are 0 become 1, and those that are 1 become 0.
    ///
    /// ## Examples
    ///
    /// Negate the arithmetice expression
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: i32 = r.expr(7)
    ///         .bit_not()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: i32 = r.bit_not(r.expr(7))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == -8 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_not(&self, number: impl Into<CommandArg>) -> Command {
        number.into().to_cmd().bit_not()
    }

    /// Compute the left arithmetic shift (left logical shift) of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_sal(param_number) → number
    /// r.bit_sal(param_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// In an arithmetic shift (also referred to as signed shift),
    /// like a logical shift, the bits that slide off the end disappear
    /// (except for the last, which goes into the carry flag).
    /// But in an arithmetic shift, the spaces are filled in such a way
    /// to preserve the sign of the number being slid. For this reason,
    /// arithmetic shifts are better suited for signed numbers in two’s
    /// complement format.
    ///
    /// ## Note
    ///
    /// SHL and SAL are the same, and differentiation only happens because
    /// SAR and SHR (right shifting) has differences in their implementation.
    ///
    /// ## Examples
    ///
    /// Compute the left arithmetic shift of 5 and 4
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr(5)
    ///         .bit_sar(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: u8 = r.bit_sar(r.expr(5), r.expr(4))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 80 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sar](Self::bit_sar)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_sal<S, T>(&self, number1: S, number2: T) -> Command
    where
        S: Into<CommandArg>,
        T: Into<CommandArg>,
    {
        number1.into().to_cmd().bit_sal(number2)
    }

    /// Compute the right arithmetic shift of one or more values.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// number.bit_sar(param_number) → number
    /// r.bit_sar(param_number, param_number) → number
    /// ```
    ///
    /// Where:
    /// - param_number: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// In an arithmetic shift (also referred to as signed shift),
    /// like a logical shift, the bits that slide off the end disappear
    /// (except for the last, which goes into the carry flag).
    /// But in an arithmetic shift, the spaces are filled in such
    /// a way to preserve the sign of the number being slid.
    /// For this reason, arithmetic shifts are better suited for
    /// signed numbers in two’s complement format.
    ///
    /// ## Examples
    ///
    /// Compute the right arithmetic shift of 32 and 3
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: u8 = r.expr(32)
    ///         .bit_sar(3)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response2: u8 = r.bit_sar(r.expr(32), r.expr(3))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 4 && response == response2);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_xor](Self::bit_xor)
    pub fn bit_sar<S, T>(&self, number1: S, number2: T) -> Command
    where
        S: Into<CommandArg>,
        T: Into<CommandArg>,
    {
        number1.into().to_cmd().bit_sar(number2)
    }

    /// Return a time object representing the current time in UTC.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.now() → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    ///
    /// # Description
    ///
    /// The command now() is computed once when the server receives the query,
    /// so multiple instances of r.now() will always return the same time inside a query.
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use neor::types::Time;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let time1: Time = r.now().value();
    ///     let time2: Time = r.now()
    ///         .cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time1.is_valid());
    ///     assert!(time2.is_valid());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    /// - [iso8601](Self::iso8601)
    pub fn now(&self) -> DateTime {
        DateTime::now()
    }

    /// Create a time object for a specific time.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.time(args!(date, timezone)) → time
    /// r.time(args!(date, time_, timezone)) → time
    /// ```
    ///
    /// Where:
    /// - date: [time::Date](time::Date)
    /// - time_: [time::Time](time::Time)
    /// - timezone: [time::UtcOffset](time::UtcOffset)
    /// - time: [Time](crate::types::Time)
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use neor::types::Time;
    /// use neor::{args, r, Converter, Result};
    /// use time::macros::{date, offset, time};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date = date!(1986 - 11 - 3);
    ///     let time = time!(09:30:40);
    ///     let timezone = offset!(+01:00);
    ///
    ///     let date_time = r.time(args!(date, time, timezone));
    ///     let time1 = date_time.value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [iso8601](Self::iso8601)
    pub fn time(&self, args: impl cmd::time::TimeArg) -> DateTime {
        DateTime::time(args)
    }

    /// Create a time object based on seconds since epoch.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.epoch_time(i64) → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use neor::types::Time;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date_time = r.epoch_time(531360000)?;
    ///     let time1 = date_time.value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [iso8601](Self::iso8601)
    pub fn epoch_time(&self, timestamp: i64) -> Result<DateTime> {
        DateTime::epoch_time(timestamp)
    }

    /// Create a time object based on an ISO 8601
    /// date-time string (e.g. ‘2013-01-01T01:01:01+00:00’).
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.iso8601(string) → time
    /// r.iso8601(args!(string, default_timezone)) → time
    /// ```
    ///
    /// Where:
    /// - time: [Time](crate::types::Time)
    /// - default_timezone: [UtcOffset](time::UtcOffset)
    ///
    /// # Description
    ///
    /// RethinkDB supports all valid ISO 8601 formats except for week dates.
    /// Read more about the ISO 8601 format at
    /// [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// If you pass an ISO 8601 string without a time zone,
    /// you must specify the time zone with the default_timezone argument.
    ///
    /// ## Examples
    ///
    /// Create a time
    ///
    /// ```
    /// use neor::types::Time;
    /// use neor::{args, r, Converter, Result};
    /// use time::macros::offset;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let date_time = r.iso8601(args!("1986-11-03T08:30:00", offset!(+01:00)))?;
    ///     let time1 = date_time.value();
    ///     let time2: Time = date_time.cmd()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(time2 == time1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    pub fn iso8601(&self, args: impl cmd::iso8601::Iso8601) -> Result<DateTime> {
        DateTime::iso8601(args)
    }

    /// Take one or more values as arguments and return an array.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.array(values) -> number
    /// ```
    ///
    /// Where:
    /// - values: `impl IntoIterator<Item = T>`
    /// - T: `impl Into<Serialize>` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Create an array.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let data = vec![1u8, 2, 3, 4];
    ///     let conn = r.connection().connect().await?;
    ///     let response: Vec<u8> = r.array(data.iter().map(|value| r.expr(value)))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == data);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [hash_map](Self::hash_map)
    pub fn array<S, T>(&self, values: T) -> Command
    where
        S: Into<CommandArg>,
        T: IntoIterator<Item = S>,
    {
        cmd::array::new(values)
    }

    /// Convert `HashMap` to object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.hash_map(value) -> number
    /// ```
    ///
    /// Where:
    /// - value: `HashMap<Key, Value>`
    /// - Key: `impl Into<String>` | [Command](crate::Command)
    /// - Value: `impl Into<Serialize>` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Create a simple object.
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use serde::{Deserialize, Serialize};
    ///
    /// use neor::{r, Converter, Result};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    /// pub struct Post {
    ///     pub id: u8,
    ///     pub title: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let expected_post = Post { id: 1, title: "post 1".to_string() };
    ///     let conn = r.connection().connect().await?;
    ///     let mut post = HashMap::new();
    ///     post.insert("id", r.expr(&expected_post.id));
    ///     post.insert("title", r.expr(&expected_post.title));
    ///
    ///     let response: Post = r.hash_map(post).run(&conn).await?.unwrap().parse()?;
    ///
    ///     assert_eq!(response, expected_post);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [array](Self::array)
    pub fn hash_map<K, V>(&self, value: HashMap<K, V>) -> Command
    where
        K: Into<CommandArg>,
        V: Into<CommandArg>,
    {
        cmd::hash_map::new(value)
    }

    /// `r.args` is a special term that’s used to splice
    /// an array of arguments into another term.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.args(array) → special
    /// ```
    /// Where:
    /// - array: `IntoIterator<Item = impl Into<String>>`
    ///
    /// # Description
    ///
    /// This is useful when you want to call a variadic term such as
    /// [get_all](crate::Command::get_all)
    /// with a set of arguments produced at runtime.
    ///
    /// Note that `args` evaluates all its arguments before passing them
    /// into the parent term, even if the parent term otherwise allows lazy evaluation.
    ///
    /// ## Examples
    ///
    /// Unpack array
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let data = vec![1, 2, 3];
    ///
    ///     let response: Vec<u8> = r.args(&data)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn args<T, S>(&self, values: T) -> Command
    where
        S: Serialize,
        T: IntoIterator<Item = S> + Serialize,
    {
        cmd::args::new(values)
    }

    /// Encapsulate binary data within a query.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.binary(data) → binary
    /// ```
    ///
    /// Where:
    /// - data: &[u8]
    /// - binary: [Binary](crate::types::Binary)
    ///
    /// # Description
    ///
    /// Binary struct returned to the client in Rust.
    /// This can be changed with the `binary_format` option
    /// provided to [run](crate::Command::run) to return “raw” objects.
    ///
    /// Only a limited subset of ReQL commands may be chained after `binary`:
    /// - [coerce_to](crate::Command::coerce_to) can coerce binary objects to string types
    /// - [count](Self::count) will return the number of bytes in the object
    /// - [slice](crate::Command::slice) will treat bytes like array indexes
    /// (i.e., slice(args!(10,20)) will return bytes 10–19)
    /// - [type_of](crate::Command::type_of) returns `TypeOf::PtypeBinary`
    /// - [info](Self::info) will return information on a binary struct.
    ///
    /// ## Examples
    ///
    /// Save an avatar image to a existing user record.
    ///
    /// ```
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let avatar_img = std::fs::read("default_avatar.png")?;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("images")
    ///         .insert(r.binary(&avatar_img))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Get the size of an existing avatar image.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: usize = r.table("images")
    ///         .get(100)
    ///         .g("avatar")
    ///         .count(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 14156);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Read more details about RethinkDB’s binary object support:
    /// [Storing binary objects]("https://rethinkdb.com/docs/storing-binary/python/").
    pub fn binary(&self, data: &[u8]) -> Binary {
        cmd::binary::new(data)
    }

    /// Call an anonymous function using return values
    /// from other ReQL commands or queries as arguments.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// any.do_(predicate) → any
    /// r.do_(arg, predicate) → any
    /// r.do_(args, predicate) → any
    /// ```
    /// Where:
    /// - predicate: [Func](crate::Func) | [Command](crate::Command) | `impl Serialize`
    /// - arg: [Command](crate::Command)
    /// - args: `impl IntoIterator<Item = T>`
    /// - T: `impl Serialize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The last argument to `do_` (or, in some forms, the only argument) is an
    /// expression or an anonymous function which receives values from either
    /// the previous arguments or from prefixed commands chained before `do_`.
    /// The `do_` command is essentially a single-element [map](Self::map),
    /// letting you map a function over just one document.
    /// This allows you to bind a query result to a
    /// local variable within the scope of `do_`,
    /// letting you compute the result just once and reuse it in
    /// a complex expression or in a series of ReQL commands.
    ///
    /// Arguments passed to the `do_` function must be basic data types,
    /// and cannot be streams or selections.
    /// ([Read about ReQL data types](https://rethinkdb.com/docs/data-types/).)
    /// While the arguments will all be evaluated before the function is executed,
    /// they may be evaluated in any order, so their values should not be dependent on one another.
    /// The type of `do_`’s result is the type of the value returned from the function or last expression.
    ///
    /// ## Examples
    ///
    /// Compute a golfer’s net score for a game.
    ///
    /// ```
    /// use neor::{func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("players")
    ///         .get("86be93eb-a112-48f5-a829-15b2cb49de1d")
    ///         .do_(func!(|player| player.g("gross_score") - player.g("course_handicap")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return the name of the best scoring player in a two-player golf match.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let (id1, id2) = (1, 2);
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.do_(
    ///         [r.table("players").get(id1), r.table("players").get(id2)],
    ///         func!(|player1, player2| r.branch(
    ///             player1.g("gross_score").lt(player2.g("gross_score")),
    ///             args!(player1, player2)
    ///         ))
    ///     )
    ///     .run(&conn)
    ///     .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Note that `branch`, the ReQL conditional command, must be used instead of `if`.
    /// See the `branch` [documentation](Self::branch) for more.
    ///
    /// ## Examples
    ///
    /// Take different actions based on the result of a ReQL insert command.
    ///
    /// ```
    /// use neor::{args, func, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let new_data = json!({
    ///         "id": 100,
    ///         "name": "Agatha",
    ///         "gross_score": 57,
    ///         "course_handicap": 4
    ///     });
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("players")
    ///     .insert(new_data)
    ///     .do_(func!(|doc| r.branch(
    ///         doc.g("inserted").ne(0),
    ///         args!(
    ///             r.table("log").insert(json!({"time": r.now(), "result": "ok"})),
    ///             r.table("log").insert(json!({"time": r.now(), "result": "error"}))
    ///         )
    ///     )))
    ///     .run(&conn)
    ///     .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    pub fn do_<A, E>(&self, args: A, expr: E) -> Command
    where
        A: cmd::do_::DoArg,
        E: Into<CommandArg>,
    {
        args.into_do_opts().add_to_cmd(cmd::do_::new(expr))
    }

    /// Perform a branching conditional equivalent to `if-then-else`.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.branch(test, args!(true_action, false_action)) → any
    /// r.branch(test, args!(true_action, tests, false_action)) → any
    /// query.branch(args!(true_action, false_action)) -> any
    /// query.branch(args!(true_action, tests, false_action)) → any
    /// ```
    ///
    /// Where:
    /// - action, true_action, false_action: `impl Serialize` | [Command](crate::Command)
    /// - test: `bool` | [Command](crate::Command)
    /// - tests: `impl IntoIterator<Item = (test, action)>` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// The `branch` command takes 2n+1 arguments: pairs of conditional expressions
    /// and commands to be executed if the conditionals return any value but `false`
    /// or `None` i.e., “truthy” values), with a final “else” command to be evaluated
    /// if all of the conditionals are `false` or `None`.
    ///
    /// You may call `branch` infix style on the first test.
    /// (See the second example for an illustration.)
    ///
    /// ```text
    /// r.branch(test1, args!(val1, [(test2, val2)], elseval))
    /// ```
    ///
    /// is the equivalent of the Rust statement
    ///
    /// ```text
    /// if (test1) {
    ///     val1
    /// } else if (test2) {
    ///     val2
    /// } else {
    ///     elseval
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Test the value of x.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let x = 10;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.branch(x > 5, args!("big", "small"))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("big"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// As above, infix-style.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let x = 10;
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.expr(x > 5)
    ///         .branch(args!("big", "small"))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("big"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Categorize heroes by victory counts.
    ///
    /// ```
    /// use std::ops::Add;
    ///
    /// use neor::{args, func, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("pricings")
    ///         .map(func!(|offer| r.branch(
    ///             offer.g("price").gt(100),
    ///             args!(
    ///                 offer.g("offer").add("premium"),
    ///                 [(
    ///                     offer.g("price").gt(10),
    ///                     offer.g("offer").add("standard")
    ///                 )],
    ///                 offer.g("offer").add("freemium")
    ///         ))))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [do_](Self::do_)
    pub fn branch<T, A>(&self, test: T, args: A) -> Command
    where
        T: Into<CommandArg>,
        A: cmd::branch::BranchArg,
    {
        test.into().to_cmd().branch(args)
    }

    /// Generate a stream of sequential integers in a specified range.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.range(()) → stream
    /// r.range(end_value) → stream
    /// r.range(args!(start_value, end_value)) → stream
    /// ```
    ///
    /// Where
    /// - start_value, end_value: `i8, u8, ..., isize, usize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// `range` takes 0, 1 or 2 arguments:
    /// - With no arguments, `range` returns an “infinite” stream
    /// from 0 up to and including the maximum integer value;
    /// - With one argument, `range` returns a stream from
    /// 0 up to but not including the end value;
    /// - With two arguments, `range` returns a stream from
    /// the start value up to but not including the end value.
    ///
    /// Note that the left bound (including the implied left
    /// bound of 0 in the 0- and 1-argument form)
    /// is always closed and the right bound is always open:
    /// the start value will always be included in the returned range
    /// and the end value will **not** be included in the returned range.
    ///
    /// Any specified arguments must be integers, or a `ReqlRuntimeError` will be thrown.
    /// If the start value is equal or to higher than the end value,
    /// no error will be thrown but a zero-element stream will be returned.
    ///
    /// ## Examples
    ///
    /// Return a four-element range of `[0, 1, 2, 3]`.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 4] = r.range(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [0, 1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// You can also use the [limit](crate::Command::limit)
    /// command with the no-argument
    /// variant to achieve the same result in this case:
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 4] = r.range(())
    ///         .limit(4)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [0, 1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Return a range from -5 through 5.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [i8; 11] = r.range(args!(-5, 6))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn range(&self, args: impl cmd::range::RangeArg) -> Command {
        cmd::range::new(args)
    }

    /// Throw a runtime error.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.error(message) → value
    /// ```
    ///
    /// Where:
    /// - message: `impl Into<String>` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// If called with no arguments inside the second
    /// argument to default, re-throw the current error.
    ///
    /// ## Examples
    ///
    /// Get Error
    ///
    /// ```
    /// use neor::err::{ReqlError, ReqlRuntimeError,};
    /// use neor::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let msg = "Error";
    ///
    ///     let err = r.error(msg).run(&conn).await.err().unwrap();
    ///
    ///     if let ReqlError::Runtime(err) = err {
    ///         if let ReqlRuntimeError::User(err) = err {
    ///             assert!(err == msg);
    ///     
    ///             return Ok(());
    ///         }
    ///     }
    ///
    ///     assert!(false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn error(&self, message: impl Into<CommandArg>) -> Command {
        cmd::error::new(message)
    }

    /// Construct a ReQL JSON object from a native object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.expr(value) → value
    /// ```
    ///
    /// Where:
    /// - value: `impl Serialize` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// If the native object is of the `Binary` type,
    /// then expr will return a binary object.
    /// See [binary](Self::binary) for more information.
    ///
    /// ## Examples
    ///
    /// Objects wrapped with `expr` can then be manipulated by ReQL API functions.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let data = [1, 2, 3];
    ///
    ///     let response: [u8; 3] = r.expr(data)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn expr(&self, value: impl Into<CommandArg>) -> Command {
        cmd::expr::new(value)
    }

    /// Create a javascript expression.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.js(js_string) → String
    /// r.js(args!(js_string, options)) → String
    /// ```
    ///
    /// Where:
    /// - js_string: `impl String` | [Command](crate::Command)
    /// - options: [JsOption](crate::arguments::JsOption)
    ///
    /// # Description
    ///
    /// ```text
    /// Whenever possible, you should use native ReQL
    /// commands rather than r.js for better performance.
    /// ```
    ///
    /// ## Examples
    ///
    /// Concatenate two strings using JavaScript.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.js("'str1' + 'str2'")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("str1str2"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// You may also specify a timeout in seconds (defaults to 5).
    ///
    /// ```
    /// use neor::arguments::JsOption;
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.js(args!(
    ///             "while(true) {}",
    ///             JsOption::default().timeout(1.3)
    ///         ))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn js(&self, args: impl cmd::js::JsArg) -> Command {
        cmd::js::new(args)
    }

    /// Get information about a ReQL value.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// any.info() → response
    /// r.info(any) → response
    /// ```
    ///
    /// Where:
    /// - any: [Command](crate::Command)
    /// - response: [InfoResponse](crate::types::InfoResponse)
    ///
    /// ## Examples
    ///
    /// Get information about a table such as primary key, or cache size.
    ///
    /// ```
    /// use neor::types::{InfoResponse, TypeOf};
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: InfoResponse = r.table("simbad")
    ///         .info()
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.typ == TypeOf::Table);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn info(&self, any: Command) -> Command {
        any.info()
    }

    /// Parse a JSON string on the server.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.json(json_string) → value
    /// ```
    ///
    /// Where:
    /// - json_string: `impl Into<String>` | [Command](crate::Command)
    ///
    /// ## Examples
    ///
    /// Send an array to the server.
    ///
    /// ```
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: [u8; 3] = r.json("[1,2,3]")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == [1, 2, 3]);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn json(&self, value: impl Into<CommandArg>) -> Command {
        cmd::json::new(value)
    }

    /// Retrieve data from the specified URL over HTTP.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.http(string) → value
    /// r.http(args!(string, options)) → value
    /// ```
    ///
    /// Where:
    /// - string: `impl Into<String>`
    /// - options: `impl Serialize`
    ///
    /// # Description
    ///
    /// The return type depends on the `result_format` option,
    /// which checks the `Content-Type` of the response by default.
    ///
    /// See [External API access](https://rethinkdb.com/docs/external-api-access/)
    /// for more informations
    ///
    /// ## Examples
    ///
    /// Perform an HTTP GET and store the result in a table.
    ///
    /// ```
    /// use neor::types::MutationResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: MutationResponse = r.table("simbad")
    ///         .insert(r.http("http://httpbin.org/get"))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.inserted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Perform request with parameters.
    ///
    /// ```
    /// use neor::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("http://httpbin.org/get", json!({
    ///             "params": {
    ///                 "user": 1
    ///             }
    ///         })))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Perform a `POST` request with accompanying data.
    ///
    /// ```
    /// use neor::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("http://httpbin.org/get", json!({
    ///             "method": "method",
    ///             "data": {
    ///                 "player": "Moussa",
    ///                 "game": "AURION"
    ///             }
    ///         })))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Perform a GitHub search and collect up to 3 pages of results.
    ///
    /// ```
    /// use neor::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.http(args!("https://api.github.com/search/code?q=addClass+user:mozilla", json!({
    ///             "page": "link-next",
    ///             "page_limit": 3
    ///         })))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn http<T>(&self, args: impl cmd::http::HttpArg<T>) -> Command
    where
        T: Serialize,
    {
        cmd::http::new(args)
    }

    /// Return a UUID (universally unique identifier),
    /// a string that can be used as a unique ID.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.uuid(()) → String
    /// r.uuid(value) → String
    /// ```
    ///
    /// Where:
    /// - value: `&str` | `String` | [Command](crate::Command)
    ///
    /// # Description
    ///
    /// If a string is passed to uuid as an argument,
    /// the UUID will be deterministic,
    /// derived from the string’s SHA-1 hash.
    ///
    /// RethinkDB’s UUIDs are standards-compliant.
    /// Without the optional argument,
    /// a version 4 random UUID will be generated;
    /// with that argument, a version 5 UUID will be generated,
    /// using a fixed namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`.
    /// For more information, read
    /// [Wikipedia’s UUID article](https://en.wikipedia.org/wiki/Universally_unique_identifier).
    ///
    /// ## Examples
    ///
    /// Generate a UUID.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.uuid(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Generate a UUID based on a String.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: String = r.uuid("malik@example.com")
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.eq("3461d115-2c05-5af4-9906-9f6882c58a15"));
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn uuid(&self, args: impl cmd::uuid::UuidArg) -> Command {
        cmd::uuid::new(args)
    }

    /// Construct a circular line or polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.circle(args!(point, radius)) → polygon
    /// r.circle(args!(point, radius, options)) → polygon
    /// ```
    ///
    /// Where:
    /// - radius: f64,
    /// - point: [Point](crate::types::Point)
    /// - polygon: [Polygon](crate::types::Polygon)
    ///
    /// # Description
    ///
    /// A circle in RethinkDB is a polygon or line **approximating**
    /// a circle of a given radius around a given center,
    /// consisting of a specified number of vertices (default 32).
    ///
    /// The center may be specified either by two floating point numbers, the longitude
    /// (−180 to 180) and latitude (−90 to 90) of the point on a perfect sphere
    /// (See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system), or by a point object.
    /// The radius is a floating point number whose units are meters by default,
    /// although that may be changed with the `unit` argument.
    ///
    /// ## Examples
    ///
    /// Define a point.
    ///
    /// ```
    /// use neor::types::Polygon;
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let circle: Polygon = r.circle(args!(r.point(-122.423246, 37.779388), 50.5))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 300,
    ///             "name": "Douala",
    ///             "location": circle
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    /// - [point](Self::point)
    /// - [distance](crate::Command::distance)
    pub fn circle(&self, args: impl cmd::circle::CircleArg) -> Command {
        cmd::circle::new(args)
    }

    /// Compute the distance between a point and another geometry object.
    /// At least one of the geometry objects specified must be a point.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.distance(geometry) → f64
    /// geometry.distance(args!(geometry, options)) → f64
    /// r.distance(geometry.cmd(), geometry) → f64
    /// r.distance(geometry.cmd(), args!(geometry, options)) → f64
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon)
    /// command
    /// - options: [DistanceOption](crate::arguments::DistanceOption)
    ///
    /// # Description
    ///
    /// If one of the objects is a polygon or a line, the point will be projected
    /// into the line or polygon assuming a perfect sphere model before the distance
    /// is computed (using the model specified with `geo_system`).
    /// As a consequence, if the polygon or line is extremely large compared
    /// to Earth’s radius and the distance is being computed with the default
    ///  WGS84 model, the results of `distance` should be considered approximate
    /// due to the deviation between the ellipsoid and spherical models.
    ///
    /// ## Examples
    ///
    /// Compute the distance between two points on the Earth in kilometers.
    ///
    /// ```
    /// use neor::arguments::{DistanceOption, Unit};
    /// use neor::{args, r, Converter, Geometry, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point1 = r.point(-122.423246, 37.779388);
    ///     let point2 = r.point(-117.220406, 32.719464);
    ///     let distance_option = DistanceOption::default().unit(Unit::Kilometer);
    ///
    ///     let response: f64 = r.distance(point1.cmd(), point2)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == 734.125249602186);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [polygon](crate::r::polygon)
    /// - [line](crate::r::line)
    pub fn distance(&self, geometry: Command, args: impl cmd::distance::DistanceArg) -> Command {
        geometry.distance(args)
    }

    /// Convert a [GeoJSON](https://geojson.org/) object to a ReQL geometry object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.geojson(geojson) → geometry
    /// ```
    ///
    /// Where:
    /// - geojson: [GeoJson](crate::types::GeoJson),
    /// - geometry: [ReqlGeoJson](crate::types::ReqlGeoJson)
    ///
    /// # Description
    ///
    /// RethinkDB only allows conversion of GeoJSON objects
    /// which have ReQL equivalents: Point, LineString, and Polygon.
    ///  MultiPoint, MultiLineString, and MultiPolygon are not supported.
    /// (You could, however, store multiple points, lines and polygons
    /// in an array and use a geospatial multi index with them.)
    ///
    /// Only longitude/latitude coordinates are supported.
    /// GeoJSON objects that use Cartesian coordinates,
    /// specify an altitude, or specify their own coordinate
    /// reference system will be rejected.
    ///
    /// ## Examples
    ///
    /// Convert a GeoJSON object to a ReQL geometry object.
    ///
    /// ```
    /// use neor::types::{GeoJson, GeoType};
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let geo_json = GeoJson {
    ///         typ: GeoType::Point,
    ///         coordinates: [-122.423246, 37.779388],
    ///     };
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 1,
    ///             "name": "Yaoundé",
    ///             "location": r.geojson(geo_json)
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [to_geojson](crate::Command::to_geojson)
    pub fn geojson<T: Serialize>(&self, geojson: GeoJson<T>) -> cmd::geojson::ReqlGeoJson<T> {
        cmd::geojson::ReqlGeoJson::new(geojson)
    }

    /// Construct a geometry object of type Polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.line(points) → line
    /// ```
    ///
    /// Where:
    /// - points: &[[Point](crate::types::Point)]
    /// - line: [Polygon](crate::types::Line)
    ///
    /// # Description
    ///
    /// The line can be specified in one of two ways:
    /// - Two or more two-item arrays, specifying latitude
    /// and longitude numbers of the line’s vertices;
    /// - Two or more [Point](crate::types::Point)
    /// objects specifying the line’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90)
    /// of vertices are plotted on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// ## Examples
    ///
    /// Define a line.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 101,
    ///             "route": r.line(&[
    ///                 r.point(-122.423246, 37.779388),
    ///                 r.point(-121.886420, 37.329898),
    ///             ])
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [point](Self::point)
    /// - [polygon](Self::polygon)
    /// - [circle](Self::circle)
    pub fn line(&self, points: &[cmd::point::Point]) -> cmd::line::Line {
        cmd::line::Line::new(points)
    }

    /// Construct a geometry object of type Point.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.point(longitude, latitude) → point
    /// ```
    ///
    /// Where:
    /// - latitude: f64,
    /// - longitude: f64,
    /// - points: &[[Point](crate::types::Point)]
    ///
    /// # Description
    ///
    /// The point is specified by two floating point numbers, the longitude
    /// (−180 to 180) and latitude (−90 to 90) of the point on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// ## Examples
    ///
    /// Define a point.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 1,
    ///             "name": "Yaoundé",
    ///             "location": r.point(-122.423246, 37.779388)
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    /// - [circle](Self::circle)
    pub fn point(&self, longitude: f64, latitude: f64) -> cmd::point::Point {
        cmd::point::Point::new(longitude, latitude)
    }

    /// Construct a geometry object of type Polygon.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.polygon(points) → polygon
    /// ```
    ///
    /// Where:
    /// - points: &[[Point](crate::types::Point)]
    /// - polygon: [Polygon](crate::types::Polygon)
    ///
    /// # Description
    ///
    /// The Polygon can be specified in one of two ways:
    /// - Three or more two-item arrays, specifying latitude
    /// and longitude numbers of the polygon’s vertices;
    /// - Three or more [Point](crate::types::Point)
    /// objects specifying the polygon’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90)
    /// of vertices are plotted on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// If the last point does not specify the same coordinates as
    /// the first point, `polygon` will close the polygon by connecting them.
    /// You cannot directly construct a polygon with holes in it using `polygon`,
    /// but you can use [polygon_sub](crate::types::Polygon::polygon_sub)
    /// to use a second polygon within the interior of the first to define a hole.
    ///
    /// ## Examples
    ///
    /// Define a polygon.
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.table("geo")
    ///         .insert(json!({
    ///             "id": 101,
    ///             "rectangle": r.polygon(&[
    ///                 r.point(-122.423246, 37.779388),
    ///                 r.point(-122.423246, 37.329898),
    ///                 r.point(-121.886420, 37.329898),
    ///                 r.point(-121.886420, 37.779388),
    ///             ])
    ///         }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [point](Self::point)
    /// - [line](Self::line)
    /// - [circle](Self::circle)
    pub fn polygon(&self, points: &[cmd::point::Point]) -> cmd::polygon::Polygon {
        cmd::polygon::Polygon::new(points)
    }

    /// Tests whether two geometry objects intersect with one another.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// geometry.intersects(geometry) → bool
    /// r.intersects(geometry_command, geometry) → bool
    /// sequence.intersects(geometry) → sequence_response
    /// r.intersects(sequence, geometry) → sequence_response
    /// ```
    ///
    /// Where:
    /// - geometry: [r.point(...)](crate::r::point) |
    /// [r.line(...)](crate::r::line) |
    /// [r.polygon(...)](crate::r::polygon) |
    /// command
    /// - sequence, geometry_command: [Command](crate::Command)
    ///
    /// # Description
    ///
    /// When applied to a sequence of geometry objects, `intersects` acts as a
    /// [filter](crate::Command::filter), returning a sequence of objects from
    /// the sequence that intersect with the argument.
    ///
    /// ## Examples
    ///
    /// Is `point2` within a 2000-meter circle around `point1`?
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point1 = r.point(-117.220406, 32.719464);
    ///     let point2 = r.point(-117.206201, 32.725186);
    ///     let circle_cmd = r.circle(args!(point1, 2000.));
    ///
    ///     let response: bool = r.intersects(circle_cmd, point2)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Which of the locations in a list of parks intersect `circle`?
    ///
    /// ```
    /// use neor::arguments::{CircleOption, Unit};
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let point = r.point(-117.220406, 32.719464);
    ///     let circle_opts = CircleOption::default().unit(Unit::InternationalMile);
    ///     let circle = r.circle(args!(point, 10., circle_opts));
    ///
    ///     let response = r.intersects(r.table("parks").g("area"), circle)
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [includes](crate::Command::includes)
    /// - [get_intersecting](crate::Command::get_intersecting)
    pub fn intersects(
        self,
        geometry: Command,
        args: impl cmd::intersects::IntersectsArg,
    ) -> Command {
        geometry.intersects(args)
    }

    /// Grant or deny access permissions for a user account,
    /// globally or on a per-database or per-table basis.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// r.grant(username, permission) → response
    /// table.grant(username, permission) → response
    /// db.grant(username, permission) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - response: [GrantResponse](crate::types::GrantResponse)
    ///
    /// # Description
    ///
    /// Permissions that are not defined on a local scope will
    /// be inherited from the next largest scope.
    /// For example, a write operation on a table will first
    /// check if `write` permissions are explicitly set to `true` or `false`
    /// for that table and account combination; if they are not,
    /// the `write` permissions for the database will be used
    /// if those are explicitly set; and if neither table nor database
    /// permissions are set for that account, the global `write`
    /// permissions for that account will be used.
    ///
    /// ## Note
    ///
    /// For all accounts other than the special, system-defined `admin` account,
    /// permissions that are not explicitly set in any scope will effectively be `false`.
    /// When you create a new user account by inserting a record into the
    /// [system table](https://rethinkdb.com/docs/system-tables/#users),
    /// that account will have **no** permissions until they are explicitly granted.
    ///
    /// For a full description of permissions, read
    /// [Permissions and user accounts](https://rethinkdb.com/docs/permissions-and-accounts/).
    ///
    /// ## Examples
    ///
    /// Grant the `alima` user account read and write permissions on the `users` database.
    ///
    /// ```
    /// use neor::arguments::Permission;
    /// use neor::types::GrantResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().read(true).write(true);
    ///
    ///     let response: GrantResponse = r.db("users")
    ///         .grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Deny write permissions from the `alima` account for the `simbad` table.
    ///
    /// ```
    /// use neor::arguments::Permission;
    /// use neor::types::GrantResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().write(false);
    ///
    ///     let response: GrantResponse = r.db("users")
    ///         .table("simbad")
    ///         .grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Grant `alima` the ability to use HTTP connections.
    ///
    /// ```
    /// use neor::arguments::Permission;
    /// use neor::types::GrantResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().connect(true);
    ///
    ///     let response: GrantResponse = r.grant("alima", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Grant a `monitor` account read-only access to all databases.
    ///
    /// ```
    /// use neor::arguments::Permission;
    /// use neor::types::GrantResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let permission = Permission::default().read(true);
    ///
    ///     let response: GrantResponse = r.grant("monitor", permission)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.granted == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn grant(&self, username: &str, permission: Permission) -> Command {
        cmd::grant::new(username, permission)
    }

    /// Wait for a table or all the tables in a database to be ready.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// table.wait(()) → response
    /// db.wait(()) → response
    /// r.wait(table) → response
    /// r.wait(database) → response
    /// table.wait(options) → response
    /// db.wait(options) → response
    /// r.wait(args!(table, options)) → response
    /// r.wait(args!(database, options)) → response
    /// ```
    ///
    /// Where:
    /// - table: [r.table(...)](crate::r::table) |
    /// [query.table(...)](Self::table)
    /// - db: [r.db(...)](crate::r::db)
    /// - options: [WaitOption](crate::arguments::WaitOption)
    /// - response: [WaitResponse](crate::types::WaitResponse)
    ///
    /// # Description
    ///
    /// A table may be temporarily unavailable after creation,
    /// rebalancing or reconfiguring.
    /// The `wait` command blocks until the given
    /// table (or database) is fully up to date.
    ///
    /// ## Examples
    ///
    /// Wait on a table to be ready.
    ///
    /// ```
    /// use neor::types::WaitResponse;
    /// use neor::{r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response: WaitResponse = r.table("simbad")
    ///         .wait(())
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.ready == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// Wait on a table with timeout to be ready for reads.
    ///
    /// ```
    /// use neor::arguments::{WaitFor, WaitOption};
    /// use neor::types::WaitResponse;
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let table_command = r.db("anim").table("simbad");
    ///     let opts = WaitOption::default()
    ///         .wait_for(WaitFor::ReadyForReads)
    ///         .timeout(8000f64);
    ///
    ///     let response: WaitResponse =  r.wait(args!(table_command, opts))
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response.ready == 1);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn wait(&self, args: impl cmd::wait::WaitArg) -> Command {
        cmd::wait::new(args)
    }

    /// To specify the descending ordering.
    ///
    /// # Command syntax
    /// ```text
    /// r.asc(field)
    /// r.asc(func)
    /// ```
    ///
    /// Where:
    /// - field: `impl Into<String>` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    ///
    /// ## Example
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .order_by(args!([r.expr("id"), r.asc("character")]))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn asc(&self, args: impl Into<CommandArg>) -> Command {
        cmd::asc::new(args)
    }

    /// To specify the descending ordering.
    ///
    /// # Command syntax
    /// ```text
    /// r.desc(field)
    /// r.desc(func)
    /// ```
    ///
    /// Where:
    /// - field: `impl Into<String>` | [Command](crate::Command)
    /// - func: [Func](crate::Func)
    ///
    /// ## Example
    ///
    /// ```
    /// use neor::{args, r, Converter, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .order_by(args!([r.expr("id"), r.desc("character")]))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn desc(&self, args: impl Into<CommandArg>) -> Command {
        cmd::desc::new(args)
    }

    /// max_val are used with some commands such as `between`
    /// to specify absolute upper bounds.
    ///
    /// # Command syntax
    /// ```text
    /// r::min_val()
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .between(args!(r::min_val(), r.expr(20)))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn min_val() -> Command {
        Command::new(ql2::term::TermType::Minval)
    }

    /// max_val are used with some commands such as `between`
    /// to specify absolute upper bounds.
    ///
    /// # Command syntax
    /// ```text
    /// r::max_val()
    /// ```
    ///
    /// ## Example
    ///
    /// ```
    /// use neor::{args, r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response = r.table("simbad")
    ///         .between(args!(r.expr(10), r::max_val()))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     assert!(response.is_some());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn max_val() -> Command {
        Command::new(ql2::term::TermType::Maxval)
    }
}

pub trait Converter {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T>;
}

impl Converter for serde_json::Value {
    fn parse<T: Unpin + Serialize + DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_value(self)?)
    }
}

pub trait Geometry: Into<Command> {
    fn cmd(self) -> Command {
        self.into()
    }
}

pub trait StaticString {
    fn static_string(self) -> Cow<'static, str>;
}

impl StaticString for &'static str {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for String {
    fn static_string(self) -> Cow<'static, str> {
        Cow::from(self)
    }
}

impl StaticString for &Cow<'static, str> {
    fn static_string(self) -> Cow<'static, str> {
        match self {
            Cow::Borrowed(string) => Cow::Borrowed(*string),
            Cow::Owned(string) => Cow::Owned(string.to_owned()),
        }
    }
}
