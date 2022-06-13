use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Command, Func};
use crate::ops::{ReqlOpsSequence, ReqlOpsDocManipulation};
use crate::types::{IdentifierFormat, ReadMode, Document, Sequence};

use super::{run, ReqlOps};

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
    pub fn insert(&self, documents: &[T]) -> super::insert::InsertBuilder<T> {
        super::insert::InsertBuilder::new(documents)._with_parent(self.get_parent())
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
    pub fn get(&self, primary_key: impl Serialize) -> super::get::GetBuilder<Option<Document<T>>> {
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
    pub fn get_all(&self, values: &[impl Serialize]) -> super::get_all::GetAllBuilder<Sequence<Document<T>>> {
        super::get_all::GetAllBuilder::new(values)._with_parent(self.get_parent())
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
    ) -> super::between::BetweenBuilder<Sequence<Document<T>>> {
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

    /// Grant or deny access permissions for a user account, per-table basis.
    /// 
    /// See [r::grant](crate::r::grant) for more information
    /// 
    /// ## Example
    /// 
    /// Deny write permissions from the `chatapp` account for the `admin` table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("users")
    ///         .table::<serde_json::Value>("admin")
    ///         .grant("chatapp")
    ///         .permit_write(false)
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn grant(self, username: &str) -> super::grant::GrantBuilder {
        super::grant::GrantBuilder::new(username)._with_parent(self.get_parent())
    }

    /// Query (read and/or update) the configurations for individual tables.
    /// 
    /// The `config` command is a shorthand way to access the `table_config` 
    /// [System tables](https://rethinkdb.com/docs/system-tables/#configuration-tables). 
    /// It will return the single row from the system that corresponds to the database configuration, 
    /// as if [get](super::table::TableBuilder::get) had been called on the system table with the UUID of the table in question.
    /// 
    /// ## Example
    /// 
    /// Get the configuration for the `users` table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("users").config().run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Change the write acknowledgement requirement of the `users` table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::ReadMode;
    /// use serde_json::{json, Value};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("users").config().update(json!({ "write_acks": ReadMode::Single })).run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn config(self) -> super::config::ConfigBuilder {
        super::config::ConfigBuilder::new()._with_parent(self.into())
    }

    /// Rebalances the shards of a table. When called on a database, all the tables in that database will be rebalanced.
    /// 
    /// The `rebalance` command operates by measuring the distribution of primary keys within a table and picking split points 
    /// that will give each shard approximately the same number of documents. It won’t change the number of shards within a table, 
    /// or change any other configuration aspect for the table or the database.
    /// 
    /// A table will lose availability temporarily after rebalance is called; 
    /// use the [wait](#method.wait) command to wait for the table to become available again, 
    /// or [status](#method.status) to check if the table is available for writing.
    /// 
    /// RethinkDB automatically rebalances tables when the number of shards are increased, 
    /// and as long as your documents have evenly distributed primary keys—such as the default UUIDs—it is rarely necessary to call `rebalance` manually. 
    /// Cases where `rebalance` may need to be called include:
    /// 
    /// - Tables with unevenly distributed primary keys, such as incrementing integers
    /// - Changing a table’s primary key type
    /// - Increasing the number of shards on an empty table, then using non-UUID primary keys in that table
    /// 
    /// The return value of rebalance is an object with two fields:
    /// * `rebalanced` : the number of tables rebalanced.
    /// * `status_changes` : a list of new and old table status values. Each element of the list will be an object with two fields:
    ///     - `old_val` : The table’s [status](#method.status) value before rebalance was executed.
    ///     - `new_val` : The table’s `status` value after `rebalance` was executed. (This value will almost always indicate the table is unavailable.)
    /// 
    /// See the [status](#method.status) command for an explanation of the objects returned in the `old_val` and `new_val` fields.
    /// 
    /// ## Example
    /// 
    /// Rebalance a table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("users").rebalance().run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn rebalance(self) -> super::rebalance::RebalanceBuilder {
        super::rebalance::RebalanceBuilder::new()._with_parent(self.get_parent())
    }

    /// Reconfigure a table’s sharding and replication. Use the following methods:
    /// 
    /// * `with_shards(u8)` : the number of shards, an integer from 1-64. Required.
    /// * `with_replicas(reql_rust::types::Replicas)` : either an integer or a mapping object. Required.
    ///     - If `Replicas::Int`, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    ///     - If `Replicas::Map`,  it specifies key-value pairs of server tags and the number of replicas to assign to those servers:
    ///     `{tag1: 2, tag2: 4, tag3: 2, ...}` . For more information about server tags, read 
    ///     [Administration tools](https://rethinkdb.com/docs/administration-tools/).
    /// * `with_dry_run(bool)` : if true the generated configuration will not be applied to the table, only returned.
    /// * `with_emergency_repair(reql_rust::types::EmergencyRepair)` : Used for the Emergency Repair mode. See the separate section below.
    /// 
    /// ## Example
    /// 
    /// Rebalance a table.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::Replicas;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("superheroes")
    ///         .reconfigure()
    ///         .with_shards(2)
    ///         .with_replicas(Replicas::Int(1))
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Reconfigure a table, specifying replicas by server tags.
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use std::borrow::Cow;
    /// 
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::Replicas;
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("superheroes")
    ///         .reconfigure()
    ///         .with_shards(2)
    ///         .with_replicas(Replicas::Map {
    ///             replicas: HashMap::from([
    ///                 (Cow::from("wooster"), 1),
    ///                 (Cow::from("wayne"), 1),
    ///             ]),
    ///             primary_replica_tag: Cow::from("wooster"),
    ///         })
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn reconfigure(self) -> super::reconfigure::ReconfigureBuilder {
        super::reconfigure::ReconfigureBuilder::new()._with_parent(self.get_parent())
    }

    /// Return the status of a table.
    /// 
    /// The return value is an object providing information about the table’s shards, replicas and replica readiness states. 
    /// For a more complete discussion of the object fields, read about the `table_status` table in
    /// [System tables](https://rethinkdb.com/docs/system-tables/#status-tables).
    /// 
    /// ## Example
    /// 
    /// Get a table’s status.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("users").status().run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn status(self) -> super::status::StatusBuilder {
        super::status::StatusBuilder::new()._with_parent(self.get_parent())
    }

    /// Wait for a table to be ready. 
    /// A table may be temporarily unavailable after creation, rebalancing or reconfiguring. 
    /// The `wait` command blocks until the given table is fully up to date.
    /// 
    /// The `wait` command use two optional methods:
    /// 
    /// - `with_wait_for(reql_rust::types::WaitFor)` : a string indicating a table status to wait on before returning, one of
    /// `WaitFor::ReadyForOutdatedReads`, `WaitFor::ReadyForReads`, `WaitFor::ReadyForWrites`,
    /// `WaitFor::AllReplicasReady`. The default is `WaitFor::AllReplicasReady`.
    /// - `with_timeout(std::time::Duration)` : a number indicating maximum time, in seconds, to wait for the table to be ready.
    /// If this value is exceeded, a ReqlRuntimeError will be thrown.A value of 0 means no timeout. The default is 0 (no timeout).
    /// 
    /// The return value is an object consisting of a single field, ready.
    /// The value is an integer indicating the number of tables waited for.
    /// It will always be 1 when wait is called on a table.
    /// 
    /// ## Example
    /// 
    /// Wait on a table to be ready.
    /// 
    /// ```
    /// use std::time::Duration;
    /// 
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::Value;
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Value>("superheroes")
    ///         .wait()
    ///         .with_timeout(Duration::from_secs(10))
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn wait(self) -> super::wait::WaitBuilder {
        super::wait::WaitBuilder::new()._with_parent(self.get_parent())
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<Document<T>> for TableBuilder<T> { }
impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsDocManipulation for TableBuilder<T> { }

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOps for TableBuilder<T> {
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
