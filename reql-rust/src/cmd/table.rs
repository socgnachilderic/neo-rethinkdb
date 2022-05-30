use std::marker::PhantomData;

use crate::document::Document;
use crate::ops::ReqlOpsSequence;
use crate::sequence::Sequence;
use crate::types::{IdentifierFormat, ReadMode};
use crate::{Command, Func};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{run, SuperOps};

#[derive(Debug, Clone)]
pub struct TableBuilder<T>(
    pub(crate) Command,
    pub(crate) TableOption,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct TableOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

impl<T: Unpin + Serialize + DeserializeOwned> TableBuilder<T> {
    pub(crate) fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::Table).with_arg(args);

        Self(command, TableOption::default(), PhantomData)
    }

    pub async fn run(&self, arg: impl run::Arg) -> crate::Result<Option<Sequence<Document<T>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(&self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Sequence<Document<T>>>> {
        self.get_parent()
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Sequence<Document<T>>>(arg)
    }

    pub fn with_read_mode(mut self, read_mode: ReadMode) -> Self {
        self.1.read_mode = Some(read_mode);
        self
    }

    pub fn with_identifier_format(mut self, identifier_format: IdentifierFormat) -> Self {
        self.1.identifier_format = Some(identifier_format);
        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
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
    ///     let _ = r.table::<serde_json::Value>("comments")
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
    ///     let _ = r.table::<serde_json::Value>("comments")
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
    ///     let _ = r.table::<serde_json::Value>("places")
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
    ///     let _ = r.table::<serde_json::Value>("comments")
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
    ///     let _ = r.table::<serde_json::Value>("posts")
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
    ///     let _ = r.table::<serde_json::Value>("networks")
    ///         .index_create("towers")
    ///         .with_geo(true)
    ///         .with_multi(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_create(&self, index_name: &str) -> super::index_create::IndexCreateBuilder {
        super::index_create::IndexCreateBuilder::new(index_name)._with_parent(self.get_parent())
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
    ///         .table::<serde_json::Value>("dc_universe")
    ///         .index_drop("code_name")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_drop(&self, index_name: &str) -> super::index_drop::IndexDropBuilder {
        super::index_drop::IndexDropBuilder::new(index_name)._with_parent(self.get_parent())
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
    ///         .table::<serde_json::Value>("dc_universe")
    ///         .index_list()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_list(&self) -> super::index_list::IndexListBuilder {
        super::index_list::IndexListBuilder::new()._with_parent(self.get_parent())
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
    ///         .table::<serde_json::Value>("comments")
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
    ///         .table::<serde_json::Value>("users")
    ///         .index_rename("mail", "email")
    ///         .with_overwrite(true)
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_rename(
        &self,
        old_index_name: &str,
        new_index_name: &str,
    ) -> super::index_rename::IndexRenameBuilder {
        super::index_rename::IndexRenameBuilder::new(old_index_name, new_index_name)
            ._with_parent(self.get_parent())
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
    ///     let _ = r.table::<serde_json::Value>("users")
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
    ///     let _ = r.table::<serde_json::Value>("users")
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
    ///     let _ = r.table::<serde_json::Value>("users")
    ///         .index_status()
    ///         .with_indexes(&vec!["mail", "author_name"])
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_status(&self) -> super::index_status::IndexStatusBuilder {
        super::index_status::IndexStatusBuilder::new()._with_parent(self.get_parent())
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
    ///     let _ = r.table::<serde_json::Value>("users")
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
    ///     let _ = r.table::<serde_json::Value>("users")
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
    ///     let _ = r.table::<serde_json::Value>("users")
    ///         .index_wait()
    ///         .with_indexes(&vec!["mail", "author_name"])
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn index_wait(&self) -> super::index_wait::IndexWaitBuilder {
        super::index_wait::IndexWaitBuilder::new()._with_parent(self.get_parent())
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
    pub fn set_write_hook(&self, func: Func) -> super::set_write_hook::SetWriteHookBuilder {
        super::set_write_hook::SetWriteHookBuilder::new(func)._with_parent(self.get_parent())
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
    ///     let _ = r.table::<serde_json::Value>("comments")
    ///         .get_write_hook()
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_write_hook(&self) -> super::get_write_hook::GetWriteBuilder {
        super::get_write_hook::GetWriteBuilder::new()._with_parent(self.get_parent())
    }

    ///
    /// ## Example
    ///
    /// Insert a document into the table `posts`.
    ///
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Posts {
    ///     id: String,
    ///     title: String,
    ///     content: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let post = Posts {
    ///         id: "1".to_string(),
    ///         title: "Lorem ipsum".to_string(),
    ///         content: "Dolor sit amet".to_string()
    ///     };
    ///     
    ///     r.table("posts").insert(&[post]).run(&conn).await?;
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
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Posts {
    ///     title: String,
    ///     content: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let post = Posts {
    ///         title: "Lorem ipsum".to_string(),
    ///         content: "Dolor sit amet".to_string(),
    ///     };
    ///     
    ///     r.table("posts").insert(&[post]).run(&conn).await?;
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
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Users {
    ///     id: String,
    ///     email: String,
    /// }
    ///
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     let user_1 = Users {
    ///         id: "william".to_string(),
    ///         email: "william@rethinkdb.com".to_string()
    ///     };
    ///     let user_2 = Users {
    ///         id: "lara".to_string(),
    ///         email: "lara@rethinkdb.com".to_string()
    ///     };
    ///
    ///     let users = vec![user_1, user_2];
    ///     
    ///     r.table("posts").insert(&users).run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn insert(&self, document: &[T]) -> super::insert::InsertBuilder<T> {
        super::insert::InsertBuilder::new(document)._with_parent(self.get_parent())
    }

    /// `sync` ensures that writes on a given table are written to permanent storage.
    /// Queries that specify soft durability (durability='soft') do not give such guarantees,
    /// so `sync` can be used to ensure the state of these queries.
    /// A call to sync does not return until all previous writes to the table are persisted.
    ///
    /// If successful, the operation returns an object: {"synced": 1}.
    ///
    /// ## Example
    ///
    /// After having updated multiple heroes with soft durability,
    /// we now want to wait until these changes are persisted.
    ///
    /// ```
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<serde_json::Value>("comments").sync().run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn sync(&self) -> super::sync::SyncBuilder {
        super::sync::SyncBuilder::new()._with_parent(self.get_parent())
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
    ///     let _ = r.table::<serde_json::Value>("posts")
    ///         .get("a9849eef-7176-4411-935b-79a6e3c56a74")
    ///         .run(&session).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get(&self, primary_key: impl Serialize) -> super::get::GetBuilder<T> {
        super::get::GetBuilder::new(primary_key)._with_parent(self.get_parent())
    }

    /// Get all documents where the given value matches the value of the requested index.
    ///
    /// ## Example
    ///
    /// Secondary index keys are not guaranteed to be unique so we cannot query via [get](#method.get) when using a secondary index.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<serde_json::Value>("posts")
    ///         .get_all(&["man_of_steel"])
    ///         .with_index("code_name")
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// Without an index argument, we default to the primary index.
    /// While `get` will either return the document or `None` when no document with such a primary key value exists,
    /// this will return either a one or zero length stream.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<serde_json::Value>("posts")
    ///         .get_all(&["superman"])
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Example
    ///
    /// You can get multiple documents in a single call to `get_all`.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<serde_json::Value>("posts")
    ///         .get_all(&["superman", "ant man"])
    ///         .run(&session)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_all(&self, index_keys: &[&str]) -> super::get_all::GetAllBuilder<T> {
        assert!(index_keys.len() > 0);
        super::get_all::GetAllBuilder::new(index_keys)._with_parent(self.get_parent())
    }

    /// Get all documents between two keys. Accepts three options methods:
    /// [with_index](super::between::BetweenBuilder::with_index),
    /// [with_left_bound](super::between::BetweenBuilder::with_left_bound), and
    /// [with_right_bound](super::between::BetweenBuilder::with_right_bound).
    /// If `index` is set to the name of a secondary index, `between` will return all documents where that
    /// index’s value is in the specified range (it uses the primary key by default).
    /// `left_bound` or `right_bound` may be set to `open` or `closed` to indicate whether or not
    /// to include that endpoint of the range (by default, `left_bound` is closed and `right_bound` is open).
    pub fn between(
        &self,
        lower_key: impl Serialize,
        upper_key: impl Serialize,
    ) -> super::between::BetweenBuilder<T> {
        super::between::BetweenBuilder::new(lower_key, upper_key)._with_parent(self.get_parent())
    }

    pub fn do_(&self, func: Func) -> super::do_::DoBuilder {
        super::do_::DoBuilder::new(func)._with_parent(self.get_parent())
    }

    /// Orders the result based on the given column.
    ///
    /// Argument can either be a string, `r.asc("column")` for ascending or `r.desc("column")` for descending.
    /// If the given argument is a string, the direction will default to ascending.
    ///
    /// ## Example
    ///
    /// Sort the result in descending order based on the `created_at` column.
    /// ```
    /// use reql_rust::{r, Result, Session};
    /// use reql_rust::prelude::*;
    /// 
    /// async fn example() -> Result<()> {
    ///     let mut conn = r.connection().connect().await?;
    ///     
    ///     r.table::<serde_json::Value>("posts").order_by().run(&conn).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn order_by(&self) -> super::order_by::OrderByBuilder<T> {
        super::order_by::OrderByBuilder::new()._with_parent(self.get_parent())
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for TableBuilder<T> { }

impl<T: Unpin + Serialize + DeserializeOwned> SuperOps for TableBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}

impl<T> Into<Command> for TableBuilder<T> {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_table() {
        let query = r.table::<serde_json::Value>("foo").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,["foo"]]"#;
        assert_eq!(serialised, expected);
    }

    #[test]
    fn r_db_table() {
        let query = r.db("foo").table::<serde_json::Value>("bar").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[15,[[14,["foo"]],"bar"]]"#;
        assert_eq!(serialised, expected);
    }
}
