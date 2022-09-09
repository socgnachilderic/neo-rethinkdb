use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::types::{Durability, ReturnChanges, AnyParam};
use crate::Command;

pub(crate) fn new(args: impl ReplaceArg) -> Command {
    let (arg, opts) = args.into_replace_opts();
    Command::new(TermType::Replace).with_arg(arg).with_opts(opts)
}

pub trait ReplaceArg {
    fn into_replace_opts(self) -> (Command, ReplaceOption);
}

impl ReplaceArg for AnyParam {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.into(), Default::default())
    }
}

impl ReplaceArg for Command {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self, Default::default())
    }
}

impl ReplaceArg for (AnyParam, ReplaceOption) {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.0.into(), self.1)
    }
}

impl ReplaceArg for (Command, ReplaceOption) {
    fn into_replace_opts(self) -> (Command, ReplaceOption) {
        (self.0, self.1)
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

// TODO write test