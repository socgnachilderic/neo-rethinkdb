use crate::{Command, CommandArg};
use ql2::term::TermType;

pub(crate) fn new(index_name: impl Into<CommandArg>) -> Command {
    index_name.into().add_to_cmd(TermType::IndexDrop)
}
