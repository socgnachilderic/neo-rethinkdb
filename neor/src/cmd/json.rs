use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(value: impl Into<CommandArg>) -> Command {
    value.into().add_to_cmd(TermType::Json)
}
