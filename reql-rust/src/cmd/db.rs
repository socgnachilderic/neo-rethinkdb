use crate::Command;
use ql2::term::TermType;

use super::{
    table_create::TableCreateBuilder,
    table_drop::TableDropBuilder,
    table_list::TableListBuilder,
    table::TableBuilder,
};

pub struct DbBuilder(Command);

impl DbBuilder {
    pub fn new(db_name: &str) -> Self {
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
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// See [r::table_create](crate::r::table_create) for more details.
    /// 
    pub fn table_create(self, table_name: &str) -> TableCreateBuilder {
        TableCreateBuilder::new(table_name)._with_parent(self.0)
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
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// See [r::table_create](crate::r::table_create) for more details.
    /// 
    pub fn table_drop(self, table_name: &str) -> TableDropBuilder {
        TableDropBuilder::new(table_name)._with_parent(self.0)
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
    ///         .run(&session)
    ///         .try_next().await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn table_list(self) -> TableListBuilder {
        TableListBuilder::new()._with_parent(self.0)
    }

    pub fn table(self, table_name: &str) -> TableBuilder {
        TableBuilder::new(table_name)._with_parent(self.0)
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
