use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new(args: impl Into<CommandArg>) -> Command {
    args.into().add_to_cmd(TermType::Difference)
}
