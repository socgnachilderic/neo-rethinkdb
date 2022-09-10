use ql2::term::TermType;

use crate::types::AnyParam;
use crate::{Command, Func};

pub(crate) fn new(args: impl ReduceArg) -> Command {
    let (sequence, func) = args.into_reduce_opts();
    let mut command = Command::new(TermType::Reduce);

    if let Some(seq) = sequence {
        command = command.with_arg(seq)
    }

    command.with_arg(func)
}

pub trait ReduceArg {
    fn into_reduce_opts(self) -> (Option<Command>, Command);
}

impl ReduceArg for Func {
    fn into_reduce_opts(self) -> (Option<Command>, Command) {
        (None, self.0)
    }
}

impl ReduceArg for (Command, Func) {
    fn into_reduce_opts(self) -> (Option<Command>, Command) {
        let Func(func) = self.1;

        (Some(self.0), func)
    }
}

impl ReduceArg for (AnyParam, Func) {
    fn into_reduce_opts(self) -> (Option<Command>, Command) {
        let Func(func) = self.1;

        (Some(self.0.into()), func)
    }
}

// TODO write test
