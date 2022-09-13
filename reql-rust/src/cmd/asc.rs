use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl AscArg) -> Command {
    Command::new(TermType::Asc).with_arg(args.into_asc_opts())
}

pub trait AscArg {
    fn into_asc_opts(self) -> Command;
}

impl AscArg for &str {
    fn into_asc_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl AscArg for Func {
    fn into_asc_opts(self) -> Command {
        self.0
    }
}

// TODO write test
