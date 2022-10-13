use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl FloorArg) -> Command {
    let mut command = Command::new(TermType::Floor);

    if let Some(arg) = args.into_floor_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait FloorArg {
    fn into_floor_opts(self) -> Option<Command>;
}

impl FloorArg for () {
    fn into_floor_opts(self) -> Option<Command> {
        None
    }
}

impl FloorArg for Command {
    fn into_floor_opts(self) -> Option<Command> {
        Some(self)
    }
}

impl FloorArg for f32 {
    fn into_floor_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

impl FloorArg for f64 {
    fn into_floor_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}
