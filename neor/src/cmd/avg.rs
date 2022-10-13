use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(args: impl AvgArg) -> Command {
    let mut command = Command::new(TermType::Avg);

    if let Some(arg) = args.into_avg_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait AvgArg {
    fn into_avg_opts(self) -> Option<Command>;
}

impl AvgArg for () {
    fn into_avg_opts(self) -> Option<Command> {
        None
    }
}

impl AvgArg for &str {
    fn into_avg_opts(self) -> Option<Command> {
        let arg = Command::from_json(self);

        Some(arg)
    }
}

impl AvgArg for String {
    fn into_avg_opts(self) -> Option<Command> {
        let arg = Command::from_json(self);

        Some(arg)
    }
}

impl AvgArg for Func {
    fn into_avg_opts(self) -> Option<Command> {
        Some(self.0)
    }
}

impl AvgArg for Command {
    fn into_avg_opts(self) -> Option<Command> {
        Some(self)
    }
}
