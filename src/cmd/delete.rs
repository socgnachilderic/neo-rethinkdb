use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::{Durability, ReturnChanges};
use crate::Command;

pub(crate) fn new(args: impl DeleteArg) -> Command {
    Command::new(TermType::Delete).with_opts(args.into_delete_opts())
}

pub trait DeleteArg {
    fn into_delete_opts(self) -> DeleteOption;
}

impl DeleteArg for () {
    fn into_delete_opts(self) -> DeleteOption {
        Default::default()
    }
}
impl DeleteArg for DeleteOption {
    fn into_delete_opts(self) -> DeleteOption {
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DeleteOption {
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
