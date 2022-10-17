use ql2::term::TermType;

use crate::arguments::{Args, ReplaceOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl ReplaceArg) -> Command {
    let (arg, opts) = args.into_replace_opts();

    arg.add_to_cmd(TermType::Replace).with_opts(opts)
}

pub trait ReplaceArg {
    fn into_replace_opts(self) -> (CommandArg, ReplaceOption);
}

impl<T> ReplaceArg for T
where
    T: Into<CommandArg>,
{
    fn into_replace_opts(self) -> (CommandArg, ReplaceOption) {
        (self.into(), Default::default())
    }
}

impl<T> ReplaceArg for Args<(T, ReplaceOption)>
where
    T: Into<CommandArg>,
{
    fn into_replace_opts(self) -> (CommandArg, ReplaceOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
