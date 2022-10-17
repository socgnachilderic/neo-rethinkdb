use ql2::term::TermType;

use crate::arguments::{Args, DuringOption};
use crate::types::DateTime;
use crate::Command;

pub(crate) fn new(args: impl DuringArg) -> Command {
    let (arg1, arg2, opts) = args.into_during_opts();

    Command::new(TermType::During)
        .with_arg(arg1)
        .with_arg(arg2)
        .with_opts(opts)
}

pub trait DuringArg {
    fn into_during_opts(self) -> (Command, Command, DuringOption);
}

impl DuringArg for Args<(DateTime, DateTime)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0.into(), self.0 .1.into(), Default::default())
    }
}

impl DuringArg for Args<(DateTime, DateTime, DuringOption)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0.into(), self.0 .1.into(), self.0 .2)
    }
}

impl DuringArg for Args<(DateTime, DateTime, Option<DuringOption>)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (
            self.0 .0.into(),
            self.0 .1.into(),
            self.0 .2.unwrap_or_default(),
        )
    }
}

impl DuringArg for Args<(Command, Command)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0, self.0 .1, Default::default())
    }
}

impl DuringArg for Args<(Command, Command, DuringOption)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}
