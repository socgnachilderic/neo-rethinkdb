use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(args: impl MapArg) -> Command {
    let (sequence, func) = args.into_map_opts();

    let mut command = Command::new(TermType::Map);

    for arg in sequence {
        command = command.with_arg(arg)
    }

    command.with_arg(func)
}

pub trait MapArg {
    fn into_map_opts(self) -> (Vec<Command>, Command);
}

impl MapArg for Func {
    fn into_map_opts(self) -> (Vec<Command>, Command) {
        (Vec::new(), self.0)
    }
}

impl MapArg for (Command, Func) {
    fn into_map_opts(self) -> (Vec<Command>, Command) {
        let Func(func) = self.1;

        (vec![self.0], func)
    }
}

impl MapArg for (Vec<Command>, Func) {
    fn into_map_opts(self) -> (Vec<Command>, Command) {
        let Func(func) = self.1;

        (self.0, func)
    }
}

// TODO write test
