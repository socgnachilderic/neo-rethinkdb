use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl WithoutArg) -> Command {
    args.into_without_opts()
        .add_to_cmd(Command::new(TermType::Without))
}

pub trait WithoutArg {
    fn into_without_opts(self) -> CmdOpts;
}

impl<T> WithoutArg for T
where
    T: Into<CommandArg>,
{
    fn into_without_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into().to_cmd())
    }
}

impl<T> WithoutArg for Args<T>
where
    T: IntoIterator<Item = Command>,
{
    fn into_without_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().collect())
    }
}
