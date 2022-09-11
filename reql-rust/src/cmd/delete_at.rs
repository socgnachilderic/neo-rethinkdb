use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl DeleteAtArg) -> Command {
    let (start_offset, end_offset) = args.into_delete_at_opts();
    let mut command = Command::new(TermType::DeleteAt).with_arg(start_offset);

    if let Some(end_offset) = end_offset {
        command = command.with_arg(end_offset);
    }

    command
}

pub trait DeleteAtArg {
    fn into_delete_at_opts(self) -> (Command, Option<Command>);
}

impl DeleteAtArg for isize {
    fn into_delete_at_opts(self) -> (Command, Option<Command>) {
        (Command::from_json(self), None)
    }
}

impl DeleteAtArg for (isize, isize) {
    fn into_delete_at_opts(self) -> (Command, Option<Command>) {
        (Command::from_json(self.0), Some(Command::from_json(self.0)))
    }
}

// TODO write test
