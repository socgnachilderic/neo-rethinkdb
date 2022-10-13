use ql2::term::TermType;

use crate::arguments::{Args, BetweenOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl BetweenArg) -> Command {
    let (min_key, max_key, opts) = args.into_between_opts();

    Command::new(TermType::Between)
        .with_arg(min_key.to_cmd())
        .with_arg(max_key.to_cmd())
        .with_opts(opts)
}

pub trait BetweenArg {
    fn into_between_opts(self) -> (CommandArg, CommandArg, BetweenOption);
}

impl<L, U> BetweenArg for Args<(L, U)>
where
    L: Into<CommandArg>,
    U: Into<CommandArg>,
{
    fn into_between_opts(self) -> (CommandArg, CommandArg, BetweenOption) {
        (self.0 .0.into(), self.0 .1.into(), Default::default())
    }
}

impl<L, U> BetweenArg for Args<(L, U, BetweenOption)>
where
    L: Into<CommandArg>,
    U: Into<CommandArg>,
{
    fn into_between_opts(self) -> (CommandArg, CommandArg, BetweenOption) {
        (self.0 .0.into(), self.0 .1.into(), self.0 .2)
    }
}
