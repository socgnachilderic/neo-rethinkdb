use ql2::term::TermType;

use crate::arguments::Args;
use crate::{Command, CommandArg};

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

impl SplitArg for &str {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(Command::from_json(self)), None)
    }
}

impl SplitArg for String {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(Command::from_json(self)), None)
    }
}

impl SplitArg for Command {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(self), None)
    }
}

impl<S, M> SplitArg for Args<(S, M)>
where
    S: Into<CommandArg>,
    M: Into<CommandArg>,
{
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (
            Some(self.0 .0.into().to_cmd()),
            Some(self.0 .1.into().to_cmd()),
        )
    }
}
