use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl DoArg) -> Command {
    let (args, func) = args.into_do_opts();
    let mut command = Command::new(TermType::Funcall);

    for arg in args {
        command = command.with_arg(arg)
    }

    command.with_arg(func)
}

pub trait DoArg {
    fn into_do_opts(self) -> (Vec<Command>, Command);
}

impl DoArg for Func {
    fn into_do_opts(self) -> (Vec<Command>, Command) {
        (Vec::new(), self.0)
    }
}

impl DoArg for (Command, Func) {
    fn into_do_opts(self) -> (Vec<Command>, Command) {
        let Func(func) = self.1;

        (vec![self.0], func)
    }
}

impl DoArg for (Vec<Command>, Func) {
    fn into_do_opts(self) -> (Vec<Command>, Command) {
        let Func(func) = self.1;

        (self.0, func)
    }
}

// TODO write test
