pub mod add;
pub mod and;
pub mod append;
pub mod args;
pub mod asc;
pub mod avg;
pub mod between;
pub mod binary;
pub mod bit_and;
pub mod bit_not;
pub mod bit_or;
pub mod bit_sal;
pub mod bit_sar;
pub mod bit_xor;
pub mod bracket;
pub mod branch;
pub mod ceil;
pub mod change_at;
pub mod changes;
pub mod circle;
pub mod coerce_to;
pub mod concat_map;
pub mod config;
pub mod connect;
pub mod contains;
pub mod count;
pub mod date;
pub mod day;
pub mod day_of_week;
pub mod day_of_year;
pub mod db;
pub mod db_create;
pub mod db_drop;
pub mod db_list;
pub mod default;
pub mod delete;
pub mod delete_at;
pub mod desc;
pub mod difference;
pub mod distance;
pub mod distinct;
pub mod div;
pub mod do_;
pub mod downcase;
pub mod during;
pub mod epoch_time;
pub mod eq;
pub mod eq_join;
pub mod error;
pub mod expr;
pub mod fill;
pub mod filter;
pub mod floor;
pub mod fold;
pub mod for_each;
pub(crate) mod func;
pub mod ge;
pub mod geojson;
pub mod get;
pub mod get_all;
pub mod get_field;
pub mod get_intersecting;
pub mod get_nearest;
pub mod get_write_hook;
pub mod grant;
pub mod group;
pub mod gt;
pub mod has_fields;
pub mod hours;
pub mod http;
pub mod in_timezone;
pub mod includes;
pub mod index;
pub mod index_create;
pub mod index_drop;
pub mod index_list;
pub mod index_rename;
pub mod index_status;
pub mod index_wait;
pub mod info;
pub mod inner_join;
pub mod insert;
pub mod insert_at;
pub mod intersects;
pub mod is_empty;
pub mod iso8601;
pub mod js;
pub mod json;
pub mod keys;
pub mod le;
pub mod limit;
pub mod line;
pub mod literal;
pub mod lt;
pub mod map;
pub mod match_;
pub mod max;
pub mod merge;
pub mod min;
pub mod minutes;
pub mod month;
pub mod mul;
pub mod ne;
pub mod not;
pub mod now;
pub mod nth;
pub mod object;
pub mod offsets_of;
pub mod or;
pub mod order_by;
pub mod outer_join;
pub mod pluck;
pub mod point;
pub mod polygon;
pub mod polygon_sub;
pub mod prepend;
pub mod random;
pub mod range;
pub mod rebalance;
pub mod reconfigure;
pub mod reduce;
pub mod rem;
pub mod replace;
pub mod round;
pub mod run;
pub mod sample;
pub mod seconds;
pub mod set_difference;
pub mod set_insert;
pub mod set_intersection;
pub mod set_union;
pub mod set_write_hook;
pub mod skip;
pub mod slice;
pub mod splice_at;
pub mod split;
pub mod status;
pub mod sub;
pub mod sum;
pub mod sync;
pub mod table;
pub mod table_create;
pub mod table_drop;
pub mod table_list;
pub mod time;
pub mod time_of_day;
pub mod timezone;
pub mod to_epoch_time;
pub mod to_geojson;
pub mod to_iso8601;
pub mod to_json;
pub mod type_of;
pub mod ungroup;
pub mod union;
pub mod upcase;
pub mod update;
pub mod uuid;
pub mod values;
pub mod wait;
pub mod with_fields;
pub mod without;
pub mod year;
pub mod zip;

use crate::{Command, Func};
use futures::stream::Stream;
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::str;

pub use crate::proto::Arg;

pub trait ReqlDbTableManipulatingOps: Into<Command> {
    /// Create a table
    ///
    /// A RethinkDB table is a collection of JSON documents.
    ///
    /// ## Example
    ///
    /// Create a table named "dc_universe" with the default settings.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table_create("dc_universe")
    ///         .run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// See [r::table_create](crate::r::table_create) for more details.
    /// 
    fn table_create(self, table_name: &str) -> table_create::TableCreateBuilder {
        table_create::TableCreateBuilder::new(table_name)._with_parent(self.into())
    }

    /// Drop a table from a database. The table and all its data will be deleted.
    /// 
    /// ## Example
    /// 
    /// Drop a table named “dc_universe”.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table_drop("dc_universe")
    ///         .run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// See [r::table_create](crate::r::table_create) for more details.
    /// 
    fn table_drop(self, table_name: &str) -> table_drop::TableDropBuilder {
        table_drop::TableDropBuilder::new(table_name)._with_parent(self.into())
    }

    /// List all table names in a default database. The result is a list of strings.
    /// 
    /// # Example
    /// 
    /// List all tables of the ‘marvel’ database.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("marvel").table_list()
    ///         .run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn table_list(self) -> table_list::TableListBuilder {
        table_list::TableListBuilder::new()._with_parent(self.into())
    }
}

pub trait ReqlTableManipulatingOps: Into<Command> {
    /// Create a new secondary index on a table.
    /// Secondary indexes improve the speed of many read queries at
    /// the slight cost of increased storage space and decreased write performance.
    /// For more information about secondary indexes, read the article
    /// “[Using secondary indexes in RethinkDB](https://rethinkdb.com/docs/secondary-indexes/python/).”
    ///
    /// RethinkDB supports different types of secondary indexes:
    ///
    /// - ***Simple indexes*** based on the value of a single field.
    /// - ***Compound indexes*** based on multiple fields.
    /// - ***Multi indexes*** based on arrays of values,
    /// created when passed `true` to the [with_multi](crate::cmd::index_create::IndexCreateBuilder::with_multi) method.
    /// - ***Geospatial indexes*** based on indexes of geometry objects,
    /// created when passed `true` to the [with_geo](crate::cmd::index_create::IndexCreateBuilder::with_geo) method.
    /// - Indexes based on ***arbitrary expressions***.
    ///
    /// you can pass the [with_func](crate::cmd::index_create::IndexCreateBuilder::with_func)
    /// method as a parameter to the `func!` macro to index nested fields
    /// for more details, please refer to the [doc](https://rethinkdb.com/api/java/index_create)
    ///
    /// If successful, `index_create` will return an object of the form `{"created": 1}`.
    /// If an index by that name already exists on the table, a `ReqlRuntimeError` will be thrown.
    ///
    /// ## Note
    ///
    /// An index may not be immediately available after creation.
    /// If your application needs to use indexes immediately after creation,
    /// use the [index_wait](#method.index_wait) command to ensure the indexes are ready before use.
    ///
    /// ## Example
    ///
    /// Create a simple index based on the field `postId`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("comments")
    ///         .index_create("postId")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Create a simple index based on the nested field `author > name`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("comments")
    ///         .index_create("author_name")
    ///         .with_func(func!(|row| row.bracket("author").bracket("name")))
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Create a geospatial index based on the field `location`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("places")
    ///         .index_create("location")
    ///         .with_geo(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A geospatial index field should contain only geometry objects. It will work with geometry ReQL terms
    /// ([get_intersecting](#method.get_intersecting) and [get_nearest](#method.get_nearest))
    /// as well as index-specific terms ([index_status](#method.index_status), [index_wait](#method.index_wait),
    /// [index_drop](#method.index_drop) and [index_list](#method.index_list)).
    /// Using terms that rely on non-geometric ordering such as [get_all](#method.get_all),
    /// [order_by](#method.order_by) and [between](#method.between) will result in an error.
    ///
    /// ## Example
    ///
    /// Create a compound index based on the fields `postId` and `date`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("comments")
    ///         .index_create("postAndDate")
    ///         .with_func(func!(|row| [row.clone().bracket("post_id"), row.bracket("date")]))
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Create a multi index based on the field `authors`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("posts")
    ///         .index_create("authors")
    ///         .with_multi(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Create a geospatial multi index based on the field `towers`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("networks")
    ///         .index_create("towers")
    ///         .with_geo(true)
    ///         .with_multi(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_create(self, index_name: &str) -> index_create::IndexCreateBuilder {
        index_create::IndexCreateBuilder::new(index_name)._with_parent(self.into())
    }

    /// Delete a previously created secondary index of this table.
    ///
    /// ## Example
    ///
    /// Drop a secondary index named ‘code_name’.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("dc_universe")
    ///         .index_drop("code_name")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_drop(self, index_name: &str) -> index_drop::IndexDropBuilder {
        index_drop::IndexDropBuilder::new(index_name)._with_parent(self.into())
    }

    /// List all the secondary indexes of this table.
    ///
    /// ## Example
    ///
    /// List the available secondary indexes for this table.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("dc_universe")
    ///         .index_list()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_list(self) -> index_list::IndexListBuilder {
        index_list::IndexListBuilder::new()._with_parent(self.into())
    }

    /// Rename an existing secondary index on a table.
    ///
    /// If the `overwrite` option is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed.
    /// If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.
    ///
    /// The return value on success will be an object of the format `{"renamed": 1}`, or `{"renamed": 0}` if the old and new names are the same.
    ///
    /// An error will be raised if the old index name does not exist, if the new index name is already in use and
    /// `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.
    ///
    /// ## Example
    ///
    /// Rename an index on the comments table.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("comments")
    ///         .index_rename("postId", "messageId")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Rename an index on the users table, overwriting any existing index with the new name.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table("users")
    ///         .index_rename("mail", "email")
    ///         .with_overwrite(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_rename(
        self,
        old_index_name: &str,
        new_index_name: &str,
    ) -> index_rename::IndexRenameBuilder {
        index_rename::IndexRenameBuilder::new(old_index_name, new_index_name)
            ._with_parent(self.into())
    }

    /// Get the status of the specified indexes on this table, or the status of all indexes on this table if no indexes are specified.
    ///
    /// The result is an array where for each index, there will be an object like this one (shown as JSON):
    ///
    /// ```text
    /// {
    ///     "index": <indexName>,
    ///     "ready": true,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    ///
    /// or this one:
    ///
    /// ```text
    /// {
    ///     "index": <indexName>,
    ///     "ready": false,
    ///     "progress": <float>,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Get the status of all the indexes on `test`
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_status()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Get the status of the `timestamp` index
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_status()
    ///         .with_one_index("timestamp")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Get the status of the `mail` and `author_name`` indexes
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_status()
    ///         .with_indexes(&vec!["mail", "author_name"])
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_status(self) -> index_status::IndexStatusBuilder {
        index_status::IndexStatusBuilder::new()._with_parent(self.into())
    }

    /// Wait for the specified indexes on this table to be ready,
    /// or for all indexes on this table to be ready if no indexes are specified.
    ///
    /// The result is an array containing one object for each table index:
    ///
    /// ```text
    /// {
    ///     "index": <indexName>,
    ///     "ready": true,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    ///
    /// See the [index_status](#method.index_status) documentation for a description of the field values.
    ///
    /// ## Example
    ///
    /// Wait for all indexes on the table `test` to be ready
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_wait()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Wait for `timestamp` index on the table `test` to be ready
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_wait()
    ///         .with_one_index("timestamp")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Wait for `mail` and `author_name` indexes on the table `test` to be ready
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("users")
    ///         .index_wait()
    ///         .with_indexes(&vec!["mail", "author_name"])
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn index_wait(self) -> index_wait::IndexWaitBuilder {
        index_wait::IndexWaitBuilder::new()._with_parent(self.into())
    }

    /// Sets the write hook on a table or overwrites it if one already exists.
    ///
    /// The `function` can be an anonymous function with the signature
    /// `(context: object, oldVal: object, newVal: object) -> object` or a binary
    ///  representation obtained from the `function` field of [getWriteHook](#method.get_write_hook).
    /// The function must be deterministic, and so cannot use a subquery or the `r.js` command.
    ///
    /// If successful, `set_write_hook` returns an object of the following form:
    ///
    /// ## Return
    ///
    /// ```text
    /// {
    ///     "function": <binary>,
    ///     "query": "setWriteHook(function(_var1, _var2, _var3) { return ...; })",
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Create a write hook that sets `modified_at` to the current time on each write operation.
    ///
    /// ```ignore
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("comments")
    ///         .set_write_hook(None)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn set_write_hook(self, func: Func) -> set_write_hook::SetWriteHookBuilder {
        set_write_hook::SetWriteHookBuilder::new(func)._with_parent(self.into())
    }

    /// Gets the write hook of this table.
    /// If a write hook exists, the result is an object of the following form:
    ///
    /// ## Return
    ///
    /// ```text
    /// {
    ///     "function": <binary>,
    ///     "query": "setWriteHook(function(_var1, _var2, _var3) { return ...; })",
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Get the write hook for the `comments` table.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("comments")
    ///         .get_write_hook()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn get_write_hook(self) -> get_write_hook::GetWriteBuilder {
        get_write_hook::GetWriteBuilder::new()._with_parent(self.into())
    }
}

pub trait ReqlTableWritingOps: Into<Command> {
    ///
    /// ## Example
    ///
    /// Insert a document into the table `posts`.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Posts<'a> {
    ///     id: u64,
    ///     title: &'a str,
    ///     content: &'a str,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let post = Posts { id: 1, title: "Lorem ipsum", content: "Dolor sit amet" };
    ///     
    ///     r.table("heroes").insert(&post).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Return
    ///
    /// ```text
    /// {
    ///    "deleted": 0,
    ///    "errors": 0,
    ///    "inserted": 1,
    ///    "replaced": 0,
    ///    "skipped": 0,
    ///    "unchanged": 0
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Insert a document without a defined primary key into the table `posts` where the primary key is `id`.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Posts<'a> {
    ///     title: &'a str,
    ///     content: &'a str,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let post = Posts { title: "Lorem ipsum", content: "Dolor sit amet" };
    ///     
    ///     r.table("heroes").insert(&post).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Return
    ///
    /// ```text
    /// {
    ///    "deleted": 0,
    ///    "errors": 0,
    ///    "generated_keys": [
    ///        "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ///    ],
    ///    "inserted": 1,
    ///    "replaced": 0,
    ///    "skipped": 0,
    ///    "unchanged": 0
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Insert multiple documents into the table `users`.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Users<'a> {
    ///     id: &'a str,
    ///     email: &'a str,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let user_1 = Users { id: "william", email: "william@rethinkdb.com" };
    ///     let user_2 = Users { id: "lara", email: "lara@rethinkdb.com" };
    ///     
    ///     r.table("heroes").insert(&vec![&user_1, &user_2]).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn insert(self, document: &impl Serialize) -> insert::InsertBuilder {
        insert::InsertBuilder::new(document)._with_parent(self.into())
    }

    /// Update JSON documents in a table. Accepts a JSON document, 
    /// a ReQL expression, or a combination of the two.
    /// 
    /// You can use the following method to setting query:
    /// 
    /// * [with_durability(durability: reql_rust::types::Durability)](update::UpdateBuilder::with_durability)
    /// possible values are `Durability::Hard` and `Durability::Soft`. This option will override the table or query’s durability setting (set in [run](run)). 
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
    /// * [with_return_changes(return_changes: reql_rust::types::ReturnChanges)](update::UpdateBuilder::with_return_changes) :
    ///     - `ReturnChanges::Bool(true)` : return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, 
    ///         only including the documents actually updated.
    ///     - `ReturnChanges::Bool(false)` : do not return a `changes` array (the default).
    ///     - `ReturnChanges::Always"` : behave as `ReturnChanges::Bool(true)`, 
    ///         but include all documents the command tried to update whether or not the update was successful.
    /// * [with_non_atomic(non_atomic: bool)](update::UpdateBuilder::with_non_atomic)
    /// if set to `true`, executes the update and distributes the result to replicas in a non-atomic fashion. 
    /// This flag is required to perform non-deterministic updates, such as those that require
    /// * [with_ignore_write_hook(ignore_write_hook: bool)](update::UpdateBuilder::with_ignore_write_hook)
    /// If `true`, and if the user has the config permission, ignores any [write hook](set_write_hook::SetWriteHookBuilder) when performing the update.
    /// 
    /// Update returns a struct [WritingResponseType](crate::types::WritingResponseType):
    /// 
    /// ## Example
    /// 
    /// Update the status of all posts to published.
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes").insert(&json!({ "status": "published" })).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn update(self, document: &impl Serialize) -> update::UpdateBuilder {
        update::UpdateBuilder::new(document)._with_parent(self.into())
    }
    
    /// Update JSON documents in a table. Accepts a JSON document, 
    /// a ReQL expression, or a combination of the two.
    /// 
    /// See [update](#method.update) for more information
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .update_by_func(func!(|post| post.without("status")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn update_by_func(self, func: Func) -> update::UpdateBuilder {
        update::UpdateBuilder::new_by_func(func)._with_parent(self.into())
    }

    /// Replace documents in a table. Accepts a JSON document or a ReQL expression, 
    /// and replaces the original document with the new one. 
    /// The new document must have the same primary key as the original document.
    /// 
    /// The `replace` command can be used to both insert and delete documents. 
    /// If the `“replaced”` document has a primary key that doesn’t exist in the table, 
    /// the document will be inserted; if an existing document is replaced with `None`, 
    /// the document will be deleted. Since `update`, `replace` and `replace_by_func` operations are performed atomically, 
    /// this allows atomic inserts and deletes as well.
    /// 
    /// See [update](#method.update) for more informations to setting
    /// 
    /// Replace returns a struct [WritingResponseType](crate::types::WritingResponseType):
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .replace(&json!({ "id": 1; "status": "published"; }))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn replace(self, document: &impl Serialize) -> replace::ReplaceBuilder {
        replace::ReplaceBuilder::new(document)._with_parent(self.into())
    }

    /// Replace documents in a table. Accepts a JSON document or a ReQL expression, 
    /// and replaces the original document with the new one. 
    /// 
    /// See [replace](#method.replace) for more information
    /// 
    /// ## Example
    /// 
    /// Remove the field `status` from all posts.
    /// 
    /// ```ignore
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes")
    ///         .replace_by_func(func!(|post| post.without("status")))
    ///         .run(&conn)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn replace_by_func(self, func: Func) -> replace::ReplaceBuilder {
        replace::ReplaceBuilder::new_by_func(func)._with_parent(self.into())
    }

    /// Delete one or more documents from a table.
    /// 
    /// You can use the following method to setting query:
    /// 
    /// * [with_durability(durability: reql_rust::types::Durability)](update::UpdateBuilder::with_durability)
    /// possible values are `Durability::Hard` and `Durability::Soft`. This option will override the table or query’s durability setting (set in [run](run)). 
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
    /// * [with_return_changes(return_changes: reql_rust::types::ReturnChanges)](update::UpdateBuilder::with_return_changes) :
    ///     - `ReturnChanges::Bool(true)` : return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, 
    ///         only including the documents actually updated.
    ///     - `ReturnChanges::Bool(false)` : do not return a `changes` array (the default).
    ///     - `ReturnChanges::Always"` : behave as `ReturnChanges::Bool(true)`, 
    ///         but include all documents the command tried to update whether or not the update was successful.
    /// * [with_ignore_write_hook(ignore_write_hook: bool)](update::UpdateBuilder::with_ignore_write_hook)
    /// If `true`, and if the user has the config permission, ignores any [write hook](set_write_hook::SetWriteHookBuilder), 
    /// which might have prohibited the deletion.
    /// 
    /// ## Example
    /// 
    /// Delete a single document from the table `heroes` .
    /// 
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table("heroes").delete().run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn delete(self) -> delete::DeleteBuilder {
        delete::DeleteBuilder::new()._with_parent(self.into())
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

// for debug purposes only
fn bytes_to_string(bytes: &[u8]) -> String {
    if let Ok(string) = str::from_utf8(bytes) {
        return string.to_owned();
    }
    format!("{:?}", bytes)
}

impl<'a> Command {
    pub fn sync(self) -> Self {
        Self::new(TermType::Sync).with_parent(self)
    }

    pub fn get_all(self, arg: impl get_all::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn between(self, arg: impl between::Arg<'a>) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn filter(self, arg: impl filter::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn inner_join(self, arg: impl inner_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn outer_join(self, arg: impl outer_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq_join(self, arg: impl eq_join::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn zip(self) -> Self {
        Self::new(TermType::Zip).with_parent(self)
    }

    pub fn map(self, arg: impl map::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn with_fields(self, arg: impl with_fields::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn concat_map(self, arg: impl concat_map::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn skip(self, arg: impl skip::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    /// Limits the result set to the given amount.
    ///
    /// Argument can be an isize.
    ///
    /// ## Example
    ///
    /// Get 5 records.
    /// ```ignore
    /// # reql_rust::example(|r, conn| async_stream::stream! {
    /// r.db("database").table("users").limit(5).run(conn)
    /// # });
    /// ```
    ///
    pub fn limit(self, arg: impl limit::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn slice(self, arg: impl slice::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn nth(self, arg: impl nth::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn offsets_of(self, arg: impl offsets_of::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn is_empty(self) -> Self {
        Self::new(TermType::IsEmpty).with_parent(self)
    }

    pub fn union(self, arg: impl union::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sample(self, arg: impl sample::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn group(self, arg: impl group::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ungroup(self) -> Self {
        Self::new(TermType::Ungroup).with_parent(self)
    }

    pub fn reduce(self, arg: impl reduce::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fold(self, arg: impl fold::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn count(self, arg: impl count::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn sum(self, arg: impl sum::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn avg(self, arg: impl avg::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn min(self, arg: impl min::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn max(self, arg: impl max::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn distinct(self, arg: impl distinct::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn contains(self, arg: impl contains::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn pluck(self, arg: impl pluck::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn without(self, arg: impl without::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn merge(self, arg: impl merge::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn append(self, arg: impl append::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn prepend(self, arg: impl prepend::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn difference(self, arg: impl difference::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_insert(self, arg: impl set_insert::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_union(self, arg: impl set_union::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_intersection(self, arg: impl set_intersection::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn set_difference(self, arg: impl set_difference::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bracket(self, arg: impl bracket::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_field(self, arg: impl get_field::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn has_fields(self, arg: impl has_fields::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn insert_at(self, arg: impl insert_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn splice_at(self, arg: impl splice_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn delete_at(self, arg: impl delete_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn change_at(self, arg: impl change_at::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn keys(self) -> Self {
        Self::new(TermType::Keys).with_parent(self)
    }

    pub fn values(self) -> Self {
        Self::new(TermType::Values).with_parent(self)
    }

    pub fn match_(self, arg: impl match_::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn split(self, arg: impl split::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn upcase(self) -> Self {
        Self::new(TermType::Upcase).with_parent(self)
    }

    pub fn downcase(self) -> Self {
        Self::new(TermType::Downcase).with_parent(self)
    }

    pub fn and(self, arg: impl and::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn or(self, arg: impl or::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn eq(self, arg: impl eq::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ne(self, arg: impl ne::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn gt(self, arg: impl gt::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn ge(self, arg: impl ge::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn lt(self, arg: impl lt::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn le(self, arg: impl le::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn not(self, arg: impl not::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_and(self, arg: impl bit_and::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_or(self, arg: impl bit_or::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_xor(self, arg: impl bit_xor::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_not(self) -> Self {
        !self
    }

    pub fn bit_sal(self, arg: impl bit_sal::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn bit_sar(self, arg: impl bit_sar::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn round(self) -> Self {
        Self::new(TermType::Round).with_parent(self)
    }

    pub fn ceil(self) -> Self {
        Self::new(TermType::Ceil).with_parent(self)
    }

    pub fn floor(self) -> Self {
        Self::new(TermType::Floor).with_parent(self)
    }

    pub fn in_timezone(self, arg: impl in_timezone::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn timezone(self) -> Self {
        Self::new(TermType::Timezone).with_parent(self)
    }

    pub fn during(self, arg: impl during::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn date(self) -> Self {
        Self::new(TermType::Date).with_parent(self)
    }

    pub fn time_of_day(self) -> Self {
        Self::new(TermType::TimeOfDay).with_parent(self)
    }

    pub fn year(self) -> Self {
        Self::new(TermType::Year).with_parent(self)
    }

    pub fn month(self) -> Self {
        Self::new(TermType::Month).with_parent(self)
    }

    pub fn day(self) -> Self {
        Self::new(TermType::Day).with_parent(self)
    }

    pub fn day_of_week(self) -> Self {
        Self::new(TermType::DayOfWeek).with_parent(self)
    }

    pub fn day_of_year(self) -> Self {
        Self::new(TermType::DayOfYear).with_parent(self)
    }

    pub fn hours(self) -> Self {
        Self::new(TermType::Hours).with_parent(self)
    }

    pub fn minutes(self) -> Self {
        Self::new(TermType::Minutes).with_parent(self)
    }

    pub fn seconds(self) -> Self {
        Self::new(TermType::Seconds).with_parent(self)
    }

    pub fn to_iso8601(self) -> Self {
        Self::new(TermType::ToIso8601).with_parent(self)
    }

    pub fn to_epoch_time(self) -> Self {
        Self::new(TermType::ToEpochTime).with_parent(self)
    }

    pub fn binary(self, arg: impl binary::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn branch(self, arg: impl branch::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn for_each(self, arg: impl for_each::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn default(self, arg: impl default::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn coerce_to(self, arg: impl coerce_to::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn type_of(self) -> Self {
        Self::new(TermType::TypeOf).with_parent(self)
    }

    pub fn info(self) -> Self {
        Self::new(TermType::Info).with_parent(self)
    }

    pub fn to_json(self) -> Self {
        Self::new(TermType::ToJsonString).with_parent(self)
    }

    pub fn distance(self, arg: impl distance::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn fill(self) -> Self {
        Self::new(TermType::Fill).with_parent(self)
    }

    pub fn to_geojson(self) -> Self {
        Self::new(TermType::ToGeojson).with_parent(self)
    }

    pub fn get_intersecting(self, arg: impl get_intersecting::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn get_nearest(self, arg: impl get_nearest::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn includes(self, arg: impl includes::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn intersects(self, arg: impl intersects::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn polygon_sub(self, arg: impl polygon_sub::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn grant(self, arg: impl grant::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn config(self) -> Self {
        Self::new(TermType::Config).with_parent(self)
    }

    pub fn rebalance(self) -> Self {
        Self::new(TermType::Rebalance).with_parent(self)
    }

    pub fn reconfigure(self, arg: impl reconfigure::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn status(self) -> Self {
        Self::new(TermType::Status).with_parent(self)
    }

    pub fn wait(self, arg: impl wait::Arg) -> Self {
        arg.arg().into_cmd().with_parent(self)
    }

    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        Box::pin(run::new(self, arg))
    }
}

#[cfg(test)]
fn serialise(cmd: &Command) -> String {
    serde_json::to_string(&crate::proto::Query(cmd)).unwrap()
}
