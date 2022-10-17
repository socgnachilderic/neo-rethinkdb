use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl CeilArg) -> Command {
    let mut command = Command::new(TermType::Ceil);

    if let Some(arg) = args.into_ceil_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait CeilArg {
    fn into_ceil_opts(self) -> Option<Command>;
}

impl CeilArg for () {
    fn into_ceil_opts(self) -> Option<Command> {
        None
    }
}

impl CeilArg for Command {
    fn into_ceil_opts(self) -> Option<Command> {
        Some(self)
    }
}

impl CeilArg for f32 {
    fn into_ceil_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

impl CeilArg for f64 {
    fn into_ceil_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}
