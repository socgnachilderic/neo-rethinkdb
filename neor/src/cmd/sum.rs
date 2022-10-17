use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(args: impl SumArg) -> Command {
    let mut command = Command::new(TermType::Sum);

    if let Some(arg) = args.into_sum_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait SumArg {
    fn into_sum_opts(self) -> Option<Command>;
}

impl SumArg for () {
    fn into_sum_opts(self) -> Option<Command> {
        None
    }
}

impl SumArg for &str {
    fn into_sum_opts(self) -> Option<Command> {
        let arg = Command::from_json(self);

        Some(arg)
    }
}

impl SumArg for String {
    fn into_sum_opts(self) -> Option<Command> {
        let arg = Command::from_json(self);

        Some(arg)
    }
}

impl SumArg for Func {
    fn into_sum_opts(self) -> Option<Command> {
        Some(self.0)
    }
}

impl SumArg for Command {
    fn into_sum_opts(self) -> Option<Command> {
        Some(self)
    }
}
