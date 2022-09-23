use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl FilterArg) -> Command {
    let (args, opts) = args.into_filter_opts();

    args.add_to_cmd(Command::new(TermType::Filter))
        .with_opts(opts)
}

pub trait FilterArg {
    fn into_filter_opts(self) -> (CmdOpts, FilterOption);
}

impl FilterArg for Func {
    fn into_filter_opts(self) -> (CmdOpts, FilterOption) {
        (CmdOpts::Single(self.0), Default::default())
    }
}

impl FilterArg for Command {
    fn into_filter_opts(self) -> (CmdOpts, FilterOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl FilterArg for Args<(Func, FilterOption)> {
    fn into_filter_opts(self) -> (CmdOpts, FilterOption) {
        let Func(func) = self.0 .0;

        (CmdOpts::Single(func), self.0 .1)
    }
}

impl FilterArg for Args<(Command, FilterOption)> {
    fn into_filter_opts(self) -> (CmdOpts, FilterOption) {
        (CmdOpts::Single(self.0 .0), self.0 .1)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FilterOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl FilterOption {
    pub fn default_(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_filter_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_filtered: Vec<Post> = table
            .clone()
            .filter(r.expr(json!({"view": 2})))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_filtered.len() == 2);
        assert!(data_filtered.first() == data.get(3));
        assert!(data_filtered.last() == data.get(1));

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_filter_data_with_func() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_filtered: Vec<Post> = table
            .clone()
            .filter(func!(|user| user.g("view").eq(r.expr(2))))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_filtered.len() == 2);
        assert!(data_filtered.first() == data.get(3));
        assert!(data_filtered.last() == data.get(1));

        tear_down(conn, &table_name).await
    }
}
