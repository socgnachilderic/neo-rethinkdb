use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(fields: impl Into<CommandArg>) -> Command {
    fields.into().add_to_cmd(TermType::WithFields)
}
