use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::{IdentifierFormat, ReadMode};
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl TableArg) -> Command {
    let (args, opts) = args.into_table_opts();

    args.add_to_cmd(Command::new(TermType::Table))
        .with_opts(opts)
}

pub trait TableArg {
    fn into_table_opts(self) -> (CmdOpts, TableOption);
}

impl TableArg for &str {
    fn into_table_opts(self) -> (CmdOpts, TableOption) {
        let arg = Command::from_json(self);

        (CmdOpts::Single(arg), Default::default())
    }
}

impl TableArg for (&str, TableOption) {
    fn into_table_opts(self) -> (CmdOpts, TableOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), self.1)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct TableOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

// Sequence<Document<T>>
#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::cmd::table::TableOption;
    use crate::prelude::*;
    use crate::types::ReadMode;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_table() -> Result<()> {
        let conn = r.connection().connect().await?;
        let table: Vec<Value> = r
            .db("todo_app")
            .table("geo")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(table.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_table_with_options() -> Result<()> {
        let conn = r.connection().connect().await?;
        let table_options = TableOption::default().read_mode(ReadMode::Outdated);
        let table: Vec<Value> = r
            .db("todo_app")
            .table(("geo", table_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(table.len() > 0);
        Ok(())
    }
}
