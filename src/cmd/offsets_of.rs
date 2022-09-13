use ql2::term::TermType;

use crate::prelude::Func;
use crate::types::AnyParam;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl OffsetsOfArg) -> Command {
    args.into_offsets_of_opts()
        .add_to_cmd(Command::new(TermType::OffsetsOf))
}

pub trait OffsetsOfArg {
    fn into_offsets_of_opts(self) -> CmdOpts;
}

impl OffsetsOfArg for AnyParam {
    fn into_offsets_of_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl OffsetsOfArg for Func {
    fn into_offsets_of_opts(self) -> CmdOpts {
        CmdOpts::Single(self.0)
    }
}

impl OffsetsOfArg for Command {
    fn into_offsets_of_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

// TODO write test
