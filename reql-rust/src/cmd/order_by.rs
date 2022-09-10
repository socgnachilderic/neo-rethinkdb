use std::borrow::Cow;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::{types::AnyParam, Command, Func};

pub(crate) fn new(args: impl OrderByArg) -> Command {
    let (arg, opts) = args.into_order_by_opts();

    let mut command = Command::new(TermType::OrderBy);

    if let Some(arg) = arg {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait OrderByArg {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption);
}

impl OrderByArg for OrderByOption {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Default::default(), self)
    }
}

impl OrderByArg for Func {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Some(self.0), Default::default())
    }
}

impl OrderByArg for AnyParam {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Some(self.into()), Default::default())
    }
}

impl OrderByArg for Command {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Some(self), Default::default())
    }
}

impl OrderByArg for (Func, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        let Func(func) = self.0;

        (Some(func), self.1)
    }
}

impl OrderByArg for (AnyParam, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Some(self.0.into()), self.1)
    }
}

impl OrderByArg for (Command, OrderByOption) {
    fn into_order_by_opts(self) -> (Option<Command>, OrderByOption) {
        (Some(self.0), self.1)
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
    use crate::Result;
    use crate::types::AnyParam;

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
