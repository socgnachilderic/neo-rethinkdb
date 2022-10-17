use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl UuidArg) -> Command {
    let mut command = Command::new(TermType::Uuid);

    if let Some(arg) = args.into_uui_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait UuidArg {
    fn into_uui_opts(self) -> Option<Command>;
}

impl UuidArg for () {
    fn into_uui_opts(self) -> Option<Command> {
        None
    }
}

impl UuidArg for &str {
    fn into_uui_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

impl UuidArg for String {
    fn into_uui_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

impl UuidArg for Command {
    fn into_uui_opts(self) -> Option<Command> {
        Some(self)
    }
}
