use std::borrow::Cow;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::{Command, Func};

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

impl GroupArg for Vec<&str> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self
            .into_iter()
            .map(|field| Command::from_json(field))
            .collect();

        (CmdOpts::Many(args), Default::default())
    }
}

impl GroupArg for Func {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        (CmdOpts::Single(self.0), Default::default())
    }
}

impl GroupArg for (&str, GroupOption) {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), self.1)
    }
}

impl GroupArg for (Vec<&str>, GroupOption) {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self
            .0
            .into_iter()
            .map(|field| Command::from_json(field))
            .collect();

        (CmdOpts::Many(args), self.1)
    }
}

impl GroupArg for (Func, GroupOption) {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let Func(func) = self.0;

        (CmdOpts::Single(func), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GroupOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
}

// GroupStream

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::types::GroupStream;
    use crate::Result;

    #[tokio::test]
    async fn test_group_data() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: GroupStream<String, Post> =
            table.group("title").run(&conn).await?.unwrap().parse()?;

        let data_obtained = data_obtained.collect();

        assert!(data_obtained.len() == 4);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
