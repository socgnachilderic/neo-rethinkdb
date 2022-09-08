use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl IndexRenameArg) -> Command {
    let (old_index_name, new_index_name, opts) = args.into_index_rename_opts();
    let arg_1 = Command::from_json(old_index_name);
    let arg_2 = Command::from_json(new_index_name);

    Command::new(TermType::IndexRename)
        .with_arg(arg_1)
        .with_arg(arg_2)
        .with_opts(opts)
}

pub trait IndexRenameArg {
    fn into_index_rename_opts(self) -> (String, String, IndexRenameOption);
}

impl IndexRenameArg for (&str, &str) {
    fn into_index_rename_opts(self) -> (String, String, IndexRenameOption) {
        (self.0.to_string(), self.1.to_string(), Default::default())
    }
}

impl IndexRenameArg for (&str, &str, IndexRenameOption) {
    fn into_index_rename_opts(self) -> (String, String, IndexRenameOption) {
        (self.0.to_string(), self.1.to_string(), self.2)
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
        let (conn, table) = set_up("malik1").await?;
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
        let (conn, table) = set_up("malik2").await?;
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
