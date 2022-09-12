use ql2::term::TermType;

use crate::types::AnyParam;
use crate::{Command, Func};

pub(crate) fn new(args: impl DefaultArg) -> Command {
    Command::new(TermType::Default).with_arg(args.into_default_opts())
}

pub trait DefaultArg {
    fn into_default_opts(self) -> Command;
}

impl DefaultArg for Command {
    fn into_default_opts(self) -> Command {
        self
    }
}

impl DefaultArg for Func {
    fn into_default_opts(self) -> Command {
        self.0
    }
}

impl DefaultArg for AnyParam {
    fn into_default_opts(self) -> Command {
        self.into()
    }
}

// TODO write test
