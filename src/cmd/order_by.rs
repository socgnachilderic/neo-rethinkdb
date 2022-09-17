use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl OrderByArg) -> Command {
    let (args, opts) = args.into_order_by_opts();
    let mut command = Command::new(TermType::OrderBy);

    if let Some(args) = args {
        command = args.add_to_cmd(command)
    }

    command.with_opts(opts)
}

pub trait OrderByArg {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption);
}

impl OrderByArg for OrderByOption {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Default::default(), self)
    }
}

impl OrderByArg for Func {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0)), Default::default())
    }
}

impl OrderByArg for Command {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self)), Default::default())
    }
}

impl OrderByArg for Args<(Func, OrderByOption)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        let Func(func) = self.0 .0;

        (Some(CmdOpts::Single(func)), self.0 .1)
    }
}

impl OrderByArg for Args<(Command, OrderByOption)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0 .0)), self.0 .1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct OrderByOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::{args, r, Result};

    use super::OrderByOption;

    #[tokio::test]
    async fn test_order_by_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Vec<Post> = table
            .order_by(OrderByOption::default().index("id"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_order_by_title_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let order_by_option = OrderByOption::default().index("title");
        let data_obtained: Vec<Post> = table
            .order_by(args!(r.var("id"), order_by_option))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }
}
