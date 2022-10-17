use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl RoundArg) -> Command {
    let mut command = Command::new(TermType::Round);

    if let Some(arg) = args.into_round_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait RoundArg {
    fn into_round_opts(self) -> Option<Command>;
}

impl RoundArg for () {
    fn into_round_opts(self) -> Option<Command> {
        None
    }
}

impl RoundArg for Command {
    fn into_round_opts(self) -> Option<Command> {
        Some(self)
    }
}

impl RoundArg for f32 {
    fn into_round_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

impl RoundArg for f64 {
    fn into_round_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}
