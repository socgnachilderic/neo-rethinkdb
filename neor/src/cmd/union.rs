use ql2::term::TermType;

use crate::arguments::{Args, UnionOption};
use crate::command_tools::CmdOpts;
use crate::Command;

pub(crate) fn new(args: impl UnionArg) -> Command {
    let (args, opts) = args.into_union_opts();

    args.add_to_cmd(Command::new(TermType::Union))
        .with_opts(opts)
}

pub trait UnionArg {
    fn into_union_opts(self) -> (CmdOpts, UnionOption);
}

impl UnionArg for Command {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl<T> UnionArg for T
where
    T: IntoIterator<Item = Command>,
{
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (
            CmdOpts::Many(self.into_iter().collect()),
            Default::default(),
        )
    }
}

impl UnionArg for Args<(Command, UnionOption)> {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self.0 .0), self.0 .1)
    }
}

impl<T> UnionArg for Args<(T, UnionOption)>
where
    T: IntoIterator<Item = Command>,
{
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Many(self.0 .0.into_iter().collect()), self.0 .1)
    }
}
