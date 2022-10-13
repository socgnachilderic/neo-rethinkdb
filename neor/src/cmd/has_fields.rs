use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(selector: impl Into<CommandArg>) -> Command {
    selector.into().add_to_cmd(TermType::HasFields)
}
