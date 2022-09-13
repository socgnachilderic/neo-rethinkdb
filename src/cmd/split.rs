use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl SplitArg) -> Command {
    let (arg1, arg2) = args.into_split_opts();
    let mut command = Command::new(TermType::Split);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg);
    }

    if let Some(arg1) = arg2 {
        command = command.with_arg(arg1);
    }

    command
}

pub trait SplitArg {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>);
}

impl SplitArg for () {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl SplitArg for Option<&str> {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(Command::from_json(self)), None)
    }
}

impl SplitArg for (Option<&str>, usize) {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (
            Some(Command::from_json(self.0)),
            Some(Command::from_json(self.1)),
        )
    }
}

// TODO write test
