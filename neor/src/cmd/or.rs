use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::{CmdOpts, CommandArg};
use crate::Command;

pub(crate) fn new(args: impl OrArg) -> Command {
    args.into_or_opts().add_to_cmd(Command::new(TermType::Or))
}

pub trait OrArg {
    fn into_or_opts(self) -> CmdOpts;
}

impl<T> OrArg for T
where
    T: Into<CommandArg>,
{
    fn into_or_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into().to_cmd())
    }
}

impl<S, T> OrArg for Args<T>
where
    S: Into<CommandArg>,
    T: IntoIterator<Item = S>,
{
    fn into_or_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect())
    }
}
