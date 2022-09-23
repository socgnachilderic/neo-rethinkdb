use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{AnyParam, Durability, ReturnChanges};
use crate::Command;

use super::CmdOpts;
use super::func::Func;

pub(crate) fn new(args: impl UpdateArg) -> Command {
    let (args, opts) = args.into_update_opts();

    args.add_to_cmd(Command::new(TermType::Update))
        .with_opts(opts)
}

pub trait UpdateArg {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption);
}

impl UpdateArg for AnyParam {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption) {
        (CmdOpts::Single(self.into()), Default::default())
    }
}

impl UpdateArg for Command {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl UpdateArg for Func {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption) {
        (CmdOpts::Single(self.0), Default::default())
    }
}

impl UpdateArg for (AnyParam, UpdateOption) {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption) {
        (CmdOpts::Single(self.0.into()), self.1)
    }
}

impl UpdateArg for (Command, UpdateOption) {
    fn into_update_opts(self) -> (CmdOpts, UpdateOption) {
        (CmdOpts::Single(self.0), self.1)
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
