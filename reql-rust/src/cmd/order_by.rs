use std::borrow::Cow;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::{types::AnyParam, Command, Func};

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

impl OrderByArg for AnyParam {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.into())), Default::default())
    }
}

impl OrderByArg for Command {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self)), Default::default())
    }
}

impl OrderByArg for (Func, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        let Func(func) = self.0;

        (Some(CmdOpts::Single(func)), self.1)
    }
}

impl OrderByArg for (AnyParam, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0.into())), self.1)
    }
}

impl OrderByArg for (Command, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0)), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct OrderByOption {
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::types::AnyParam;
    use crate::Result;

    use super::OrderByOption;

    #[tokio::test]
    async fn test_order_by_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Vec<Post> = table
            .order_by(OrderByOption::default().index("id"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, TABLE_NAMES[0]).await
    }

    #[tokio::test]
    async fn test_order_by_title_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[1], true).await?;
        let order_by_option = OrderByOption::default().index("title");
        let data_obtained: Vec<Post> = table
            .order_by((AnyParam::new("id"), order_by_option))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, TABLE_NAMES[1]).await
    }
}
