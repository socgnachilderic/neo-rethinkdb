use crate::types::{IdentifierFormat, ReadMode};
use crate::{Command, Func};
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;

use super::{run, ReqlTableWritingOps, ReqlTableManipulatingOps};

pub struct TableBuilder(Command, TableOption);

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

        Self(command, TableOption::default())
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<serde_json::Value>> {
        self.0.with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, serde_json::Value>(arg)
            .try_next()
            .await
    }

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
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

impl ReqlTableManipulatingOps for TableBuilder { }
impl ReqlTableWritingOps for TableBuilder { }

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
