use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::{CmdOpts, CommandArg};
use crate::Command;

pub(crate) fn new(args: impl GeArg) -> Command {
    args.into_ge_opts().add_to_cmd(Command::new(TermType::Ge))
}

pub trait GeArg {
    fn into_ge_opts(self) -> CmdOpts;
}

impl<T> GeArg for T
where
    T: Into<CommandArg>,
{
    fn into_ge_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into().to_cmd())
    }
}

impl<S, T> GeArg for Args<T>
where
    S: Into<CommandArg>,
    T: IntoIterator<Item = S>,
{
    fn into_ge_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect())
    }
}
