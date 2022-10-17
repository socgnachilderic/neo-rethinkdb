use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(message: impl Into<CommandArg>) -> Command {
    message.into().add_to_cmd(TermType::Error)
}
