use ql2::term::TermType;

use crate::prelude::Func;
use crate::types::AnyParam;
use crate::Command;

pub(crate) fn new(args: impl ContainsArg) -> Command {
    let (sequence, arg) = args.into_contains_opts();
    let mut command = Command::new(TermType::Contains);

    if let Some(seq) = sequence {
        command = command.with_arg(seq)
    }

    command.with_arg(arg)
}

pub trait ContainsArg {
    fn into_contains_opts(self) -> (Option<Command>, Command);
}

impl ContainsArg for AnyParam {
    fn into_contains_opts(self) -> (Option<Command>, Command) {
        (None, self.into())
    }
}

impl ContainsArg for Func {
    fn into_contains_opts(self) -> (Option<Command>, Command) {
        (None, self.0)
    }
}

impl ContainsArg for (Command, AnyParam) {
    fn into_contains_opts(self) -> (Option<Command>, Command) {
        (Some(self.0), self.1.into())
    }
}

impl ContainsArg for (Command, Func) {
    fn into_contains_opts(self) -> (Option<Command>, Command) {
        let Func(func) = self.1;

        (Some(self.0), func)
    }
}

// TODO write test
