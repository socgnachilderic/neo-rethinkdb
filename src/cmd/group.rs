use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl GroupArg) -> Command {
    let (args, opts) = args.into_group_opts();

    args.add_to_cmd(Command::new(TermType::Group))
        .with_opts(opts)
}

pub trait GroupArg {
    fn into_group_opts(self) -> (CmdOpts, GroupOption);
}

impl GroupArg for &str {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let arg = Command::from_json(self);

        (CmdOpts::Single(arg), Default::default())
    }
}

impl<const N: usize> GroupArg for [&str; N] {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self.into_iter().map(Command::from_json).collect();

        (CmdOpts::Many(args), Default::default())
    }
}

impl GroupArg for Func {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        (CmdOpts::Single(self.0), Default::default())
    }
}

impl<const N: usize> GroupArg for [Func; N] {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self.into_iter().map(|func| func.0).collect();

        (CmdOpts::Many(args), Default::default())
    }
}

impl GroupArg for Args<(&str, GroupOption)> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let arg = Command::from_json(self.0 .0);

        (CmdOpts::Single(arg), self.0 .1)
    }
}

impl<const N: usize> GroupArg for Args<([&str; N], GroupOption)> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self.0 .0.into_iter().map(Command::from_json).collect();

        (CmdOpts::Many(args), self.0 .1)
    }
}

impl GroupArg for Args<(Func, GroupOption)> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let Func(func) = self.0 .0;

        (CmdOpts::Single(func), self.0 .1)
    }
}

impl<const N: usize> GroupArg for Args<([Func; N], GroupOption)> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let funcs = self.0 .0.into_iter().map(|func| func.0).collect();

        (CmdOpts::Many(funcs), self.0 .1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GroupOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::types::GroupedStream;
    use crate::Result;

    #[tokio::test]
    async fn test_group_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: GroupedStream<String, Post> =
            table.group("title").run(&conn).await?.unwrap().parse()?;
            
        let data_obtained = data_obtained.collect();

        assert!(data_obtained.len() == 4);

        tear_down(conn, &table_name).await
    }
}
