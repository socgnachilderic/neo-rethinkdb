use ql2::term::TermType;

use crate::arguments::{Args, UpdateOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl UpdateArg) -> Command {
    let (arg, opts) = args.into_update_opts();

    arg.add_to_cmd(TermType::Update).with_opts(opts)
}

pub trait UpdateArg {
    fn into_update_opts(self) -> (CommandArg, UpdateOption);
}

impl<T> UpdateArg for T
where
    T: Into<CommandArg>,
{
    fn into_update_opts(self) -> (CommandArg, UpdateOption) {
        (self.into(), Default::default())
    }
}

impl<T> UpdateArg for Args<(T, UpdateOption)>
where
    T: Into<CommandArg>,
{
    fn into_update_opts(self) -> (CommandArg, UpdateOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
