use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl PluckArg) -> Command {
    args.into_pluck_opts()
        .add_to_cmd(Command::new(TermType::Pluck))
}

pub trait PluckArg {
    fn into_pluck_opts(self) -> CmdOpts;
}

impl<T> PluckArg for T
where
    T: Into<CommandArg>,
{
    fn into_pluck_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into().to_cmd())
    }
}

impl<T> PluckArg for Args<T>
where
    T: IntoIterator<Item = Command>,
{
    fn into_pluck_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().collect())
    }
}
