use ql2::term::TermType;

use crate::command_tools::CommandArg;
use crate::Command;

pub(crate) fn new(index: impl Into<CommandArg>) -> Command {
    index.into().add_to_cmd(TermType::Nth)
}
