use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(number: impl Into<CommandArg>) -> Command {
    number.into().add_to_cmd(TermType::BitSar)
}
