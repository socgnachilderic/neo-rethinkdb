use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new<O, V>(offset: O, value: V) -> Command
where
    O: Into<CommandArg>,
    V: Into<CommandArg>,
{
    Command::new(TermType::ChangeAt)
        .with_arg(offset.into().to_cmd())
        .with_arg(value.into().to_cmd())
}
