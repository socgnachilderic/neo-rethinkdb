use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(step: impl Into<CommandArg>) -> Command {
    step.into().add_to_cmd(TermType::Limit)
}
