use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::types::Squash;
use crate::Command;

pub(crate) fn new(args: impl ChangesArg) -> Command {
    Command::new(TermType::Changes)
        .with_opts(args.into_changes_opts())
        .mark_change_feed()
}

pub trait ChangesArg {
    fn into_changes_opts(self) -> ChangesOption;
}

impl ChangesArg for () {
    fn into_changes_opts(self) -> ChangesOption {
        Default::default()
    }
}

impl ChangesArg for ChangesOption {
    fn into_changes_opts(self) -> ChangesOption {
        self
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct ChangesOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<Squash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changefeed_queue_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_initial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_states: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_offsets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_types: Option<bool>,
}
