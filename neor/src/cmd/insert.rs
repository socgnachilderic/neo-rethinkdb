use ql2::term::TermType;

use crate::arguments::{Args, InsertOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl InsertArg) -> Command {
    let (arg, opts) = args.into_insert_opts();

    arg.add_to_cmd(TermType::Insert).with_opts(opts)
}

pub trait InsertArg {
    fn into_insert_opts(self) -> (CommandArg, InsertOption);
}

impl<T> InsertArg for T
where
    T: Into<CommandArg>,
{
    fn into_insert_opts(self) -> (CommandArg, InsertOption) {
        (self.into(), Default::default())
    }
}

impl<T> InsertArg for Args<(T, InsertOption)>
where
    T: Into<CommandArg>,
{
    fn into_insert_opts(self) -> (CommandArg, InsertOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
