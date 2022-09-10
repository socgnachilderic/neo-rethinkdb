use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::{Command, Func};

use super::CmdOpts;

pub(crate) fn new(args: impl IndexCreateArg) -> Command {
    let (args, func, opts) = args.into_table_create_opts();

    let mut command = args
        .add_to_cmd(Command::new(TermType::IndexCreate))
        .with_opts(opts);

    if let Some(Func(func)) = func {
        command = command.with_arg(func);
    }

    command
}

pub trait IndexCreateArg {
    fn into_table_create_opts(self) -> (CmdOpts, Option<Func>, IndexCreateOption);
}

impl IndexCreateArg for &str {
    fn into_table_create_opts(self) -> (CmdOpts, Option<Func>, IndexCreateOption) {
        let arg = Command::from_json(self);

        (CmdOpts::Single(arg), None, Default::default())
    }
}

impl IndexCreateArg for (&str, Func) {
    fn into_table_create_opts(self) -> (CmdOpts, Option<Func>, IndexCreateOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), Some(self.1), Default::default())
    }
}

impl IndexCreateArg for (&str, IndexCreateOption) {
    fn into_table_create_opts(self) -> (CmdOpts, Option<Func>, IndexCreateOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), None, self.1)
    }
}

impl IndexCreateArg for (&str, Func, IndexCreateOption) {
    fn into_table_create_opts(self) -> (CmdOpts, Option<Func>, IndexCreateOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), Some(self.1), self.2)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct IndexCreateOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::types::IndexResponse;
    use crate::{r, Command, Result, Session};

    use super::IndexCreateOption;

    #[tokio::test]
    async fn test_create_index() -> Result<()> {
        let table_name = "malik1";
        let conn = r.connection().connect().await?;
        let index_created = r.table(table_name).index_create("author");

        setup(table_name, index_created, &conn).await
    }

    #[tokio::test]
    async fn test_create_index_with_options() -> Result<()> {
        let table_name = "malik2";
        let conn = r.connection().connect().await?;
        let index_option = IndexCreateOption::default().multi(true);
        let index_created = r.table(table_name).index_create(("author", index_option));

        setup(table_name, index_created, &conn).await
    }

    /* #[tokio::test]
    async fn test_create_index_with_func() -> Result<()> {
        let table_name = "malik3";
        let conn = r.connection().connect().await?;
        let index_created = r
            .table(table_name)
            .index_create(("author", func!(|row| row.bracket("author").bracket("name"))));

        setup(table_name, index_created, &conn).await
    }

    #[tokio::test]
    async fn test_create_index_with_func_and_options() -> Result<()> {
        let table_name = "malik2";
        let conn = r.connection().connect().await?;
        let index_option = IndexCreateOption::default().multi(true);
        let index_created = r.table(table_name).index_create((
            "author",
            func!(|row| row.bracket("author").bracket("name")),
            index_option,
        ));

        setup(table_name, index_created, &conn).await
    } */

    async fn setup(table_name: &str, index_created: Command, conn: &Session) -> Result<()> {
        r.table_create(table_name).run(conn).await?;

        let index_created: IndexResponse = index_created.run(conn).await?.unwrap().parse()?;

        assert!(index_created.created > Some(0));

        r.table_drop(table_name).run(conn).await?;
        Ok(())
    }
}
