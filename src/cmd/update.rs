use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, Durability, ReturnChanges};
use crate::Command;

use super::func::Func;

pub(crate) fn new(args: impl UpdateArg) -> Command {
    let (arg, opts) = args.into_update_opts();

    Command::new(TermType::Update).with_arg(arg).with_opts(opts)
}

pub trait UpdateArg {
    fn into_update_opts(self) -> (Command, UpdateOption);
}

impl<T> UpdateArg for T
where
    T: Serialize,
{
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (Command::from_json(self), Default::default())
    }
}

impl UpdateArg for Command {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self, Default::default())
    }
}

impl UpdateArg for Func {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.0, Default::default())
    }
}

impl<T> UpdateArg for Args<(T, UpdateOption)>
where
    T: Serialize,
{
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (Command::from_json(self.0 .0), self.0 .1)
    }
}

impl UpdateArg for Args<(Command, UpdateOption)> {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.0 .0, self.0 .1)
    }
}

impl UpdateArg for Args<(Func, UpdateOption)> {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.0 .0 .0, self.0 .1)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct UpdateOption {
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
    use serde_json::json;

    use crate::prelude::*;
    use crate::spec::*;
    use crate::types::MutationResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_update_docs() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse = table
            .get(1)
            .update(json!({"view": 0}))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.replaced == 1);

        tear_down(conn, &table_name).await
    }
}
