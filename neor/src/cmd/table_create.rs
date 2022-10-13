use ql2::term::TermType;

use crate::arguments::{Args, TableCreateOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl TableCreateArg) -> Command {
    let (arg, opts) = args.into_table_create_opts();

    arg.add_to_cmd(TermType::TableCreate).with_opts(opts)
}

pub trait TableCreateArg {
    fn into_table_create_opts(self) -> (CommandArg, TableCreateOption);
}

impl<T> TableCreateArg for T
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, TableCreateOption) {
        (self.into(), Default::default())
    }
}

impl<T> TableCreateArg for Args<(T, TableCreateOption)>
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, TableCreateOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
