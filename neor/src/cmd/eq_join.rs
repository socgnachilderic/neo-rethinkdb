use ql2::term::TermType;

use crate::arguments::{Args, EqJoinOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl EqJoinArg) -> Command {
    let (arg, right_table, opts) = args.into_eq_join_opts();

    arg.add_to_cmd(TermType::EqJoin)
        .with_arg(right_table)
        .with_opts(opts)
}

pub trait EqJoinArg {
    fn into_eq_join_opts(self) -> (CommandArg, Command, EqJoinOption);
}

impl<T> EqJoinArg for Args<(T, Command)>
where
    T: Into<CommandArg>,
{
    fn into_eq_join_opts(self) -> (CommandArg, Command, EqJoinOption) {
        (self.0 .0.into(), self.0 .1, Default::default())
    }
}

impl<T> EqJoinArg for Args<(T, Command, EqJoinOption)>
where
    T: Into<CommandArg>,
{
    fn into_eq_join_opts(self) -> (CommandArg, Command, EqJoinOption) {
        (self.0 .0.into(), self.0 .1, self.0 .2)
    }
}
