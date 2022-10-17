use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::{Command, Func};

pub(crate) fn new(args: impl CountArg) -> Command {
    let mut command = Command::new(TermType::Count);

    if let Some(arg) = args.into_count_arg() {
        command = command.with_arg(arg)
    }

    command
}

pub trait CountArg {
    fn into_count_arg(self) -> Option<Command>;
}

impl CountArg for () {
    fn into_count_arg(self) -> Option<Command> {
        None
    }
}

impl CountArg for Command {
    fn into_count_arg(self) -> Option<Command> {
        Some(self)
    }
}

impl CountArg for Func {
    fn into_count_arg(self) -> Option<Command> {
        Some(self.0)
    }
}

impl<T> CountArg for Args<T>
where
    T: Serialize,
{
    fn into_count_arg(self) -> Option<Command> {
        Some(Command::from_json(self.0))
    }
}
