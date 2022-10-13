use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, IdentifierFormat, ReadMode};
use crate::Command;

pub(crate) fn new(args: impl TableArg) -> Command {
    let (arg, opts) = args.into_table_opts();

    Command::new(TermType::Table).with_arg(arg).with_opts(opts)
}

pub trait TableArg {
    fn into_table_opts(self) -> (Command, TableOption);
}

impl<T> TableArg for T
where
    T: Into<String>,
{
    fn into_table_opts(self) -> (Command, TableOption) {
        (Command::from_json(self.into()), Default::default())
    }
}

impl<T> TableArg for Args<(T, TableOption)>
where
    T: Into<String>,
{
    fn into_table_opts(self) -> (Command, TableOption) {
        (Command::from_json(self.0 .0.into()), self.0 .1)
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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::arguments::ReadMode;
    use crate::cmd::table::TableOption;
    use crate::prelude::*;
    use crate::{args, r, Result};

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
            .table(args!("geo", table_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(table.len() > 0);
        Ok(())
    }
}
