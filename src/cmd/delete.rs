use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Durability, ReturnChanges};
use crate::Command;

pub(crate) fn new(args: impl DeleteArg) -> Command {
    Command::new(TermType::Delete).with_opts(args.into_delete_opts())
}

pub trait DeleteArg {
    fn into_delete_opts(self) -> DeleteOption;
}

impl DeleteArg for () {
    fn into_delete_opts(self) -> DeleteOption {
        Default::default()
    }
}
impl DeleteArg for DeleteOption {
    fn into_delete_opts(self) -> DeleteOption {
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DeleteOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_atomic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::arguments::ReturnChanges;
    use crate::prelude::Converter;
    use crate::spec::*;
    use crate::types::MutationResponse;
    use crate::Result;

    use super::DeleteOption;

    #[tokio::test]
    async fn test_delete_docs() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse =
            table.get(5).delete(()).run(&conn).await?.unwrap().parse()?;

        assert!(response.deleted == 1);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_delete_docs_with_opts() -> Result<()> {
        let data = Post::get_many_data().get(0).unwrap().to_owned();
        let delete_option = DeleteOption::default().return_changes(ReturnChanges::Bool(true));
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse = table
            .get(1)
            .delete(delete_option)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.deleted == 1);

        let old_val: Post = response
            .changes
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
            .old_val
            .unwrap()
            .parse()?;

        assert!(old_val == data);

        tear_down(conn, &table_name).await
    }
}
