use ql2::term::TermType;

use crate::types::AnyParam;
use crate::{Command, Func};

pub(crate) fn new(args: impl OffsetsOfArg) -> Command {
    Command::new(TermType::OffsetsOf).with_arg(args.into_offsets_of_opts())
}

pub trait OffsetsOfArg {
    fn into_offsets_of_opts(self) -> Command;
}

impl OffsetsOfArg for AnyParam {
    fn into_offsets_of_opts(self) -> Command {
        self.into()
    }
}

impl OffsetsOfArg for Func {
    fn into_offsets_of_opts(self) -> Command {
        self.0
    }
}

impl OffsetsOfArg for Command {
    fn into_offsets_of_opts(self) -> Command {
        self
    }
}

// TODO write test
