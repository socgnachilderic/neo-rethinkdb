use crate::{Command, CommandArg};

pub(crate) fn new(value: impl Into<CommandArg>) -> Command {
    value.into().to_cmd()
}
