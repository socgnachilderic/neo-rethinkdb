use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl DescArg) -> Command {
    Command::new(TermType::Desc).with_arg(args.into_desc_opts())
}

pub trait DescArg {
    fn into_desc_opts(self) -> Command;
}

impl DescArg for &str {
    fn into_desc_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl DescArg for Func {
    fn into_desc_opts(self) -> Command {
        self.0
    }
}

// TODO write test
