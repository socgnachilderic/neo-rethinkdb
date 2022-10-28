use std::usize;

use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg, Func};

use super::index::Index;

pub(crate) fn new(args: impl OrderByArg) -> Command {
    let (args, index) = args.into_order_by_opts();
    let mut command = Command::new(TermType::OrderBy);

    if let Some(args) = args {
        command = args.add_to_cmd(command)
    }

    if let Some(index) = index {
        command = command.with_arg(index.0)
    }

    command
}

pub trait OrderByArg {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>);
}

impl OrderByArg for Index {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (Default::default(), Some(self))
    }
}

impl<T> OrderByArg for T
where
    T: Into<String>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (
            Some(CmdOpts::Single(Command::from_json(self.into()))),
            Default::default(),
        )
    }
}

impl OrderByArg for Func {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (Some(CmdOpts::Single(self.0)), Default::default())
    }
}

impl OrderByArg for Command {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (Some(CmdOpts::Single(self)), Default::default())
    }
}

impl<T> OrderByArg for Args<(T, Index)>
where
    T: Into<CommandArg>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (
            Some(CmdOpts::Single(self.0 .0.into().to_cmd())),
            Some(self.0 .1),
        )
    }
}

impl<T, const N: usize> OrderByArg for Args<[T; N]>
where
    T: Into<CommandArg>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, Option<Index>) {
        (
            Some(CmdOpts::Many(
                self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect(),
            )),
            Default::default(),
        )
    }
}
