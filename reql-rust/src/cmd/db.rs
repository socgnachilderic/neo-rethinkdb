use crate::Command;
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct DbBuilder(pub(crate) Command);

impl DbBuilder {
    pub(crate) fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);

        Self(
            Command::new(TermType::Db)
                .with_arg(args)
                .into_arg::<()>()
                .into_cmd()
        )
    }

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
    pub fn table_create(self, table_name: &str) -> super::table_create::TableCreateBuilder {
        super::table_create::TableCreateBuilder::new(table_name)._with_parent(self.into())
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
    pub fn table_drop(self, table_name: &str) -> super::table_drop::TableDropBuilder {
        super::table_drop::TableDropBuilder::new(table_name)._with_parent(self.into())
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
    pub fn table_list(self) -> super::table_list::TableListBuilder {
        super::table_list::TableListBuilder::new()._with_parent(self.into())
    }

    /// Return all documents in a table. Other commands may be chained after `table` to return a subset of documents 
    /// (such as [get](super::get::GetBuilder) and [filter](super::filter::FilterBuilder)) or perform further processing.
    /// 
    /// ## Example
    /// 
    /// Return all documents in the table ‘marvel’ of the default database.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct Marvel {
    ///     id: String,
    ///     name: String
    /// }
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.table::<Marvel>("marvel").run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// ## Example
    /// 
    /// Return all documents in the table ‘marvel’ of the database ‘heroes’.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct Marvel {
    ///     id: String,
    ///     name: String
    /// }
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes").table::<Marvel>("marvel").run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// There are two methods that may be used to setting.
    /// 
    /// * [with_read_mode(read_mode: reql_rust::types::ReadMode)](super::table::TableBuilder::with_read_mode) :
    /// One of three possible values affecting the consistency guarantee for the table read:
    ///     - `ReadMode::Single` : returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
    ///     - `ReadMode::Majority` : will only return values that are safely committed on disk on a majority of replicas. This requires sending a message 
    ///         to every replica on each read, so it is the slowest but most consistent.
    ///     - `ReadMode::Outdated` : will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
    /// * [with_identifier_format(identifier_format: reql_rust::types::IdentifierFormat)](super::table::TableBuilder::with_identifier_format) :
    ///     - `IdentifierFormat::Name`
    ///     - `IdentifierFormat::Uuid` : then [system tables](https://rethinkdb.com/docs/system-tables/) will refer to servers,
    ///         databases and tables by UUID rather than name. (This only has an effect when used with system tables.)
    /// 
    /// ## Example
    /// 
    /// Allow potentially out-of-date data in exchange for faster reads.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::ReadMode;
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct Marvel {
    ///     id: String,
    ///     name: String
    /// }
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("heroes")
    ///         .table::<Marvel>("marvel")
    ///         .with_read_mode(ReadMode::Outdated)
    ///         .run(&session).await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn table<T>(self, table_name: &str) -> super::table::TableBuilder<T>
    where
        T: Unpin + Serialize + DeserializeOwned,
    {
        super::table::TableBuilder::new(table_name)._with_parent(self.0)
    }

    /// Grant or deny access permissions for a user account, on a per-database basis.
    /// 
    /// See [r::grant](crate::r::grant) for more information
    /// 
    /// ## Example
    /// 
    /// Grant the `chatapp` user account read and write permissions on the `users` database.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.db("users")
    ///         .grant("chatapp")
    ///         .permit_read(true)
    ///         .permit_write(true)
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn grant(self, username: &str) -> super::grant::GrantBuilder {
        super::grant::GrantBuilder::new(username)._with_parent(self.into())
    }
}

impl Into<Command> for DbBuilder {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db() {
        let query = r.db("foo").into();
        let serialised = cmd::serialise(&query);
        let expected = r#"[14,["foo"]]"#;
        assert_eq!(serialised, expected);
    }
}
