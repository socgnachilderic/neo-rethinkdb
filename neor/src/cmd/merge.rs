use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl MergeArg) -> Command {
    args.into_merge_opts()
        .add_to_cmd(Command::new(TermType::Merge))
}

pub trait MergeArg {
    fn into_merge_opts(self) -> CmdOpts;
}

impl<T> MergeArg for T
where
    T: Into<CommandArg>,
{
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into().to_cmd())
    }
}

impl<S, T> MergeArg for Args<T>
where
    S: Into<Command>,
    T: IntoIterator<Item = S>,
{
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().map(Into::into).collect())
    }
}
