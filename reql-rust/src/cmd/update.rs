use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Document;
use crate::types::{Durability, ReturnChanges};
use crate::Command;

pub(crate) fn new(args: impl UpdateArg) -> Command {
    let (arg, opts) = args.into_update_opts();
    Command::new(TermType::Update).with_arg(arg).with_opts(opts)
}

pub trait UpdateArg {
    fn into_update_opts(self) -> (Command, UpdateOption);
}

impl<T: Document> UpdateArg for T {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        let command = Command::from_json(self.get_document());

        (command, Default::default())
    }
}

impl UpdateArg for Command {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        (self, Default::default())
    }
}

impl<T: Document> UpdateArg for (T, UpdateOption) {
    fn into_update_opts(self) -> (Command, UpdateOption) {
        let command = Command::from_json(self.0.get_document());

        (command, self.1)
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