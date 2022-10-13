use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

pub(crate) fn new(args: impl IndexRenameArg) -> Command {
    let (old_index_arg, new_index_arg, opts) = args.into_index_rename_opts();

    Command::new(TermType::IndexRename)
        .with_arg(old_index_arg)
        .with_arg(new_index_arg)
        .with_opts(opts)
}

pub trait IndexRenameArg {
    fn into_index_rename_opts(self) -> (Command, Command, IndexRenameOption);
}

impl<O, N> IndexRenameArg for Args<(O, N)>
where
    O: Into<String>,
    N: Into<String>,
{
    fn into_index_rename_opts(self) -> (Command, Command, IndexRenameOption) {
        (
            Command::from_json(self.0 .0.into()),
            Command::from_json(self.0 .1.into()),
            Default::default(),
        )
    }
}

impl<O, N> IndexRenameArg for Args<(O, N, IndexRenameOption)>
where
    O: Into<String>,
    N: Into<String>,
{
    fn into_index_rename_opts(self) -> (Command, Command, IndexRenameOption) {
        (
            Command::from_json(self.0 .0.into()),
            Command::from_json(self.0 .1.into()),
            self.0 .2,
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, CommandOptions)]
pub struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::types::IndexResponse;
    use crate::{args, Result};

    use super::IndexRenameOption;

    #[tokio::test]
    async fn test_rename_index() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        table.clone().index_create("author").run(&conn).await?;
        let index_renamed: IndexResponse = table
            .clone()
            .index_rename(args!("author", "author_name"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_renamed.renamed > Some(0));

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_rename_index_with_overwrite() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        table.clone().index_create("author").run(&conn).await?;
        table.clone().index_create("author_name").run(&conn).await?;

        let index_renamed: IndexResponse = table
            .clone()
            .index_rename(args!(
                "author",
                "author_name",
                IndexRenameOption::default().overwrite(true)
            ))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_renamed.renamed > Some(0));

        tear_down(conn, &table_name).await
    }
}
