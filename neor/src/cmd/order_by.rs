use ql2::term::TermType;

use crate::arguments::{Args, OrderByOption};
use crate::command_tools::CmdOpts;
use crate::{Command, Func};

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

impl OrderByArg for Args<(Func, OrderByOption)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        let Func(func) = self.0 .0;

        (Some(CmdOpts::Single(func)), self.0 .1)
    }
}

impl OrderByArg for Args<(Command, OrderByOption)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (Some(CmdOpts::Single(self.0 .0)), self.0 .1)
    }
}

impl OrderByArg for Args<(Command, Command)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        (
            Some(CmdOpts::Many(vec![self.0 .0, self.0 .1])),
            Default::default(),
        )
    }
}

impl OrderByArg for Args<(Func, Command)> {
    fn into_order_by_opts(self) -> (Option<CmdOpts>, OrderByOption) {
        let Func(func) = self.0 .0;
        (
            Some(CmdOpts::Many(vec![func, self.0 .1])),
            Default::default(),
        )
    }
}
