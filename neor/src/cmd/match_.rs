use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(regex: impl Into<CommandArg>) -> Command {
    regex.into().add_to_cmd(TermType::Match)
}
