use std::borrow::Cow;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

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

impl GroupArg for Vec<&str> {
    fn into_group_opts(self) -> (CmdOpts, GroupOption) {
        let args = self
            .into_iter()
            .map(|field| Command::from_json(field))
            .collect();

        (CmdOpts::Many(args), Default::default())
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GroupOption {
    pub index: Option<Cow<'static, str>>,
    pub multi: Option<bool>,
}

// GroupStream
