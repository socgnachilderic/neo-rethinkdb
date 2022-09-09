use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::types::{Durability, ReturnChanges, AnyParam};
use crate::Command;

pub(crate) fn new(args: impl UpdateArg) -> Command {
    let (arg, opts) = args.into_update_opts();
    Command::new(TermType::Update).with_arg(arg).with_opts(opts)
}

pub trait UpdateArg {
    fn into_update_opts(self) -> (Command, UpdateOption);
}

impl UpdateArg for AnyParam {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.into(), Default::default())
    }
}

impl UpdateArg for Command {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self, Default::default())
    }
}

impl UpdateArg for (AnyParam, UpdateOption) {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.0.into(), self.1)
    }
}

impl UpdateArg for (Command, UpdateOption) {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self.0, self.1)
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

// TODO write test