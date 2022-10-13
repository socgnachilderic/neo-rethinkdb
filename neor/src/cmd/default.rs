use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(default_value: impl Into<CommandArg>) -> Command {
    default_value.into().add_to_cmd(TermType::Default)
}
