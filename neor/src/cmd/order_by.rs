use std::usize;

use ql2::term::TermType;

use crate::arguments::{Args, OrderByOption};
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg, Func};

pub(crate) fn new(args: impl OrderByArg) -> Command {
    let (args, opts) = args.into_order_by_opts();
    let mut command = Command::new(TermType::OrderBy);

    if let Some(args) = args {
        command = args.add_to_cmd(command)
    }

    command.with_opts(opts)
}

pub trait OrderByArg {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption);
}

impl OrderByArg for OrderByOption {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Default::default(), self)
    }
}

impl<T> OrderByArg for T
where
    T: Into<String>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (
            Some(CmdOpts::Single(Command::from_json(self.into()))),
            Default::default(),
        )
    }
}

impl OrderByArg for Func {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0)), Default::default())
    }
}

impl OrderByArg for Command {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self)), Default::default())
    }
}

impl<T> OrderByArg for Args<(T, OrderByOption)>
where
    T: Into<CommandArg>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0 .0.into().to_cmd())), self.0 .1)
    }
}

impl<T, const N: usize> OrderByArg for Args<[T; N]>
where
    T: Into<CommandArg>,
{
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (
            Some(CmdOpts::Many(
                self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect(),
            )),
            Default::default(),
        )
    }
}
