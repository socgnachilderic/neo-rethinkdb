use ql2::term::TermType;

use crate::command_tools::CmdOpts;
use crate::{Command, Geometry};

pub(crate) fn new(geometry: impl IncludesArg) -> Command {
    geometry
        .into_includes_opts()
        .add_to_cmd(Command::new(TermType::Includes))
}

pub trait IncludesArg {
    fn into_includes_opts(self) -> CmdOpts;
}

impl<T: Geometry> IncludesArg for T {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl IncludesArg for Command {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<T: Geometry> IncludesArg for Vec<T> {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Many(self.into_iter().map(|geo| geo.cmd()).collect())
    }
}

impl IncludesArg for Vec<Command> {
    fn into_includes_opts(self) -> CmdOpts {
        CmdOpts::Many(self)
    }
}
