use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, Durability, ReturnChanges};
use crate::Command;

use super::func::Func;

pub(crate) fn new(args: impl ReplaceArg) -> Command {
    let (arg, opts) = args.into_replace_opts();

    Command::new(TermType::Replace)
        .with_arg(arg)
        .with_opts(opts)
}

pub trait ReplaceArg {
    fn into_replace_opts(self) -> (Command, ReplaceOption);
}

impl<T> ReplaceArg for T
where
    T: Serialize,
{
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (Command::from_json(self), Default::default())
    }
}

impl ReplaceArg for Command {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self, Default::default())
    }
}

impl ReplaceArg for Func {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.0, Default::default())
    }
}

impl<T> ReplaceArg for Args<(T, ReplaceOption)>
where
    T: Serialize,
{
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (Command::from_json(self.0 .0), self.0 .1)
    }
}

impl ReplaceArg for Args<(Command, ReplaceOption)> {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.0 .0, self.0 .1)
    }
}

impl ReplaceArg for Args<(Func, ReplaceOption)> {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.0 .0 .0, self.0 .1)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct ReplaceOption {
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
    use crate::prelude::*;
    use crate::spec::*;
    use crate::types::MutationResponse;
    use crate::Result;

    // use super::DeleteOption;

    #[tokio::test]
    async fn test_replace_docs() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse = table
            .get(1)
            .replace(data)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.replaced == 1);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_replace_docs_with_func() -> Result<()> {
        let lenght = Post::get_many_data().len();
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse = table
            .replace(func!(|post| post.without("view")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.replaced == lenght);

        tear_down(conn, &table_name).await
    }
}
