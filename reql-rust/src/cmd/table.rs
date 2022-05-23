use crate::types::{IdentifierFormat, ReadMode};
use crate::{Command, Func};
use futures::Stream;
use ql2::term::TermType;
use serde::Serialize;

use super::run;

pub struct TableBuilder(Command, TableOption, Option<Command>);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct TableOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

impl TableBuilder {
    pub fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::Table).with_arg(args);

        Self(command, TableOption::default(), None)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<serde_json::Value>> {
        let mut cmd = self.0.with_opts(self.1);

        if let Some(parent) = self.2 {
            cmd = cmd.with_parent(parent);
        }

        let cmd = cmd.into_arg::<()>().into_cmd();

        cmd.run::<_, serde_json::Value>(arg)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.2 = Some(parent);
        self
    }
    pub fn with_read_mode(mut self, read_mode: ReadMode) -> Self {
        self.1.read_mode = Some(read_mode);
        self
    }

    pub fn with_identifier_format(mut self, identifier_format: IdentifierFormat) -> Self {
        self.1.identifier_format = Some(identifier_format);
        self
    }

    /// Turn a query into a changefeed, an infinite stream of objects
    /// representing changes to the query’s results as they occur.
    /// A changefeed may return changes to a table or an individual document (a “point” changefeed).
    /// Commands such as filter or map may be used before the changes command to transform or filter the output,
    /// and many commands that operate on sequences can be chained after changes.
    pub fn changes(self) -> super::changes::ChangesBuilder {
        super::changes::ChangesBuilder::new()._with_parent(self.0)
    }

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
    pub fn index_create(self, index_name: &str) -> super::index_create::IndexCreateBuilder {
        super::index_create::IndexCreateBuilder::new(index_name)._with_parent(self.0)
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
    pub fn index_drop(self, index_name: &str) -> super::index_drop::IndexDropBuilder {
        super::index_drop::IndexDropBuilder::new(index_name)._with_parent(self.0)
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
    pub fn index_list(self) -> super::index_list::IndexListBuilder {
        super::index_list::IndexListBuilder::new()._with_parent(self.0)
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
    pub fn index_rename(
        self,
        old_index_name: &str,
        new_index_name: &str,
    ) -> super::index_rename::IndexRenameBuilder {
        super::index_rename::IndexRenameBuilder::new(old_index_name, new_index_name)
            ._with_parent(self.0)
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
    pub fn index_status(self) -> super::index_status::IndexStatusBuilder {
        super::index_status::IndexStatusBuilder::new()._with_parent(self.0)
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
    pub fn index_wait(self) -> super::index_wait::IndexWaitBuilder {
        super::index_wait::IndexWaitBuilder::new()._with_parent(self.0)
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
    pub fn set_write_hook(self, func: Func) -> super::set_write_hook::SetWriteHookBuilder {
        super::set_write_hook::SetWriteHookBuilder::new(func)._with_parent(self.0)
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
    pub fn get_write_hook(self) -> super::get_write_hook::GetWriteBuilder {
        super::get_write_hook::GetWriteBuilder::new()._with_parent(self.0)
    }

    /// Get a document by primary key.
    ///
    /// If no document exists with that primary key, get will return `None`.
    ///
    /// ## Example
    ///
    /// Find a document by UUID.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table("posts")
    ///         .get("a9849eef-7176-4411-935b-79a6e3c56a74")
    ///         .run::<_, serde_json::Value>(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get(self, primary_key: &str) -> super::get::GetBuilder {
        super::get::GetBuilder::new(primary_key)
    }

    pub fn do_(self, func: Func) -> super::do_::DoBuilder {
        super::do_::DoBuilder::new(func)._with_parent(self.0)
    }

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
    pub fn insert(self, document: &impl Serialize) -> super::insert::InsertBuilder {
        super::insert::InsertBuilder::new(document)._with_parent(self.0)
    }

    /// Orders the result based on the given column.
    ///
    /// Argument can either be a string, `r.asc("column")` for ascending or `r.desc("column")` for descending.
    /// If the given argument is a string, the direction will default to ascending.
    ///
    /// ## Example
    ///
    /// Sort the result in descending order based on the `created_at` column.
    // ```
    // # reql_rust::example(|r, conn| async_stream::stream! {
    // r.db("database").table("users").order_by(r.desc("created_at")).run(conn)
    // # });
    // ```
    ///
    pub fn order_by(self, arg: impl super::order_by::Arg) -> Command {
        // arg.arg().into_cmd().with_parent(self)
        todo!()
    }
}

impl Into<Command> for TableBuilder {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_table() {
        let query = r.table("foo").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,["foo"]]"#;
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table() {
        let query = r.db("foo").table("bar").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,[[14,["foo"]],"bar"]]"#;
        assert_eq!(serialised, expected);
    }
}
