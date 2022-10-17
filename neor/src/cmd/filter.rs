use ql2::term::TermType;

use crate::arguments::{Args, FilterOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl FilterArg) -> Command {
    let (arg, opts) = args.into_filter_opts();

    arg.add_to_cmd(TermType::Filter).with_opts(opts)
}

pub trait FilterArg {
    fn into_filter_opts(self) -> (CommandArg, FilterOption);
}

impl<T> FilterArg for T
where
    T: Into<CommandArg>,
{
    fn into_filter_opts(self) -> (CommandArg, FilterOption) {
        (self.into(), Default::default())
    }
}

impl<T> FilterArg for Args<(T, FilterOption)>
where
    T: Into<CommandArg>,
{
    fn into_filter_opts(self) -> (CommandArg, FilterOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
