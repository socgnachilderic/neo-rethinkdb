use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(attr: impl Into<CommandArg>) -> Command {
    attr.into().add_to_cmd(TermType::Bracket)
}
