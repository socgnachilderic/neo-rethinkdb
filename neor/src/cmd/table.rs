use ql2::term::TermType;

use crate::arguments::{Args, TableOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl TableArg) -> Command {
    let (arg, opts) = args.into_table_opts();

    arg.add_to_cmd(TermType::Table).with_opts(opts)
}

pub trait TableArg {
    fn into_table_opts(self) -> (CommandArg, TableOption);
}

impl<T> TableArg for T
where
    T: Into<CommandArg>,
{
    fn into_table_opts(self) -> (CommandArg, TableOption) {
        (self.into(), Default::default())
    }
}

impl<T> TableArg for Args<(T, TableOption)>
where
    T: Into<CommandArg>,
{
    fn into_table_opts(self) -> (CommandArg, TableOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
