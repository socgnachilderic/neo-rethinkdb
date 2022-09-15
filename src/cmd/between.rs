use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::AnyParam;
use crate::types::Status;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl BetweenArg) -> Command {
    let (min_key, max_key, opts) = args.into_between_opts();
    let min_key: Option<Command> = min_key.into();
    let max_key: Option<Command> = max_key.into();

    Command::new(TermType::Between)
        .with_arg(min_key.unwrap())
        .with_arg(max_key.unwrap())
        .with_opts(opts)
}

pub trait BetweenArg {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption);
}

impl BetweenArg for (AnyParam, AnyParam) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0.into()),
            CmdOpts::Single(self.1.into()),
            Default::default(),
        )
    }
}

impl BetweenArg for (Command, AnyParam) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0),
            CmdOpts::Single(self.1.into()),
            Default::default(),
        )
    }
}

impl BetweenArg for (AnyParam, Command) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0.into()),
            CmdOpts::Single(self.1),
            Default::default(),
        )
    }
}

impl BetweenArg for (Command, Command) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0),
            CmdOpts::Single(self.1),
            Default::default(),
        )
    }
}

impl BetweenArg for (AnyParam, AnyParam, BetweenOption) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0.into()),
            CmdOpts::Single(self.1.into()),
            self.2,
        )
    }
}

impl BetweenArg for (Command, AnyParam, BetweenOption) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0),
            CmdOpts::Single(self.1.into()),
            self.2,
        )
    }
}

impl BetweenArg for (AnyParam, Command, BetweenOption) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (
            CmdOpts::Single(self.0.into()),
            CmdOpts::Single(self.1),
            self.2,
        )
    }
}

impl BetweenArg for (Command, Command, BetweenOption) {
    fn into_between_opts(self) -> (CmdOpts, CmdOpts, BetweenOption) {
        (CmdOpts::Single(self.0), CmdOpts::Single(self.1), self.2)
    }
}

#[derive(
    Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
pub struct BetweenOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[cfg(test)]
mod tests {
    use crate::arguments::AnyParam;
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::types::Status;
    use crate::{r, Result};

    use super::BetweenOption;

    #[tokio::test]
    async fn test_get_data_between() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_get: Vec<Post> = table
            .between((AnyParam::new(2), AnyParam::new(4)))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == 2);
        assert!(data_get.first() == data.get(2));
        assert!(data_get.last() == data.get(1));

        tear_down(conn, table_name.as_str()).await
    }

    #[tokio::test]
    async fn test_get_data_between_by_minval() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_get: Vec<Post> = table
            .between((r::min_val(), AnyParam::new(4)))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == 3);
        assert!(data_get.first() == data.get(2));
        assert!(data_get.last() == data.first());

        tear_down(conn, table_name.as_str()).await
    }

    #[tokio::test]
    async fn test_get_data_between_by_maxval() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_get: Vec<Post> = table
            .between((AnyParam::new(2), r::max_val()))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == 4);
        assert!(data_get.first() == data.get(3));
        assert!(data_get.last() == data.get(1));

        tear_down(conn, table_name.as_str()).await
    }

    #[tokio::test]
    async fn test_get_data_between_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let between_option = BetweenOption::default().right_bound(Status::Closed);
        let data_get: Vec<Post> = table
            .between((AnyParam::new(2), AnyParam::new(4), between_option))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == 3);
        assert!(data_get.first() == data.get(3));
        assert!(data_get.last() == data.get(1));

        tear_down(conn, table_name.as_str()).await
    }

    #[tokio::test]
    async fn test_get_data_between_by_minval_and_max_val_with_opts() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let between_option = BetweenOption::default()
            .right_bound(Status::Closed)
            .left_bound(Status::Closed)
            .index("title");
        let data_get: Vec<Post> = table
            .between((r::min_val(), r::max_val(), between_option))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == data.len());
        assert!(data_get.first() == data.get(3));
        assert!(data_get.last() == data.first());

        tear_down(conn, table_name.as_str()).await
    }
}
