use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl IndexRenameArg) -> Command {
    let (old_index_arg, new_index_arg, opts) = args.into_index_rename_opts();
    let arg_1: Option<Command> = old_index_arg.into();
    let arg_2: Option<Command> = new_index_arg.into();

    Command::new(TermType::IndexRename)
        .with_arg(arg_1.unwrap())
        .with_arg(arg_2.unwrap())
        .with_opts(opts)
}

pub trait IndexRenameArg {
    fn into_index_rename_opts(self) -> (CmdOpts, CmdOpts, IndexRenameOption);
}

impl IndexRenameArg for (&str, &str) {
    fn into_index_rename_opts(self) -> (CmdOpts, CmdOpts, IndexRenameOption) {
        let old_arg = Command::from_json(self.0);
        let new_arg = Command::from_json(self.1);

        (CmdOpts::Single(old_arg), CmdOpts::Single(new_arg), Default::default())
    }
}

impl IndexRenameArg for (&str, &str, IndexRenameOption) {
    fn into_index_rename_opts(self) -> (CmdOpts, CmdOpts, IndexRenameOption) {
        let old_arg = Command::from_json(self.0);
        let new_arg = Command::from_json(self.1);

        (CmdOpts::Single(old_arg), CmdOpts::Single(new_arg), self.2)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, CommandOptions)]
#[non_exhaustive]
pub struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::types::IndexResponse;
    use crate::Result;

    use super::IndexRenameOption;

    #[tokio::test]
    async fn test_rename_index() -> Result<()> {
        let (conn, table) = set_up("malik1", false).await?;
        table.clone().index_create("author").run(&conn).await?;
        let index_renamed: IndexResponse = table
            .clone()
            .index_rename(("author", "author_name"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_renamed.renamed > Some(0));

        tear_down(conn, "malik1").await
    }

    #[tokio::test]
    async fn test_rename_index_with_overwrite() -> Result<()> {
        let (conn, table) = set_up("malik2", false).await?;
        table.clone().index_create("author").run(&conn).await?;
        table.clone().index_create("author_name").run(&conn).await?;

        let index_renamed: IndexResponse = table
            .clone()
            .index_rename((
                "author",
                "author_name",
                IndexRenameOption::default().overwrite(true),
            ))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_renamed.renamed > Some(0));

        tear_down(conn, "malik2").await
    }
}
