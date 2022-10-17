use ql2::term::TermType;

use crate::arguments::{Args, SliceOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl SliceArg) -> Command {
    let (start_offset, end_offset, opts) = args.into_slice_opts();
    let mut command = Command::new(TermType::Slice).with_arg(start_offset.to_cmd());

    if let Some(end_offset) = end_offset {
        command = command.with_arg(end_offset.to_cmd());
    }

    command.with_opts(opts)
}

pub trait SliceArg {
    fn into_slice_opts(self) -> (CommandArg, Option<CommandArg>, SliceOption);
}

impl<T> SliceArg for T
where
    T: Into<CommandArg>,
{
    fn into_slice_opts(self) -> (CommandArg, Option<CommandArg>, SliceOption) {
        (self.into(), None, Default::default())
    }
}

impl<S, E> SliceArg for Args<(S, E)>
where
    S: Into<CommandArg>,
    E: Into<CommandArg> + Copy,
{
    fn into_slice_opts(self) -> (CommandArg, Option<CommandArg>, SliceOption) {
        (self.0 .0.into(), Some(self.0 .1.into()), Default::default())
    }
}

impl<T> SliceArg for Args<(T, SliceOption)>
where
    T: Into<CommandArg>,
{
    fn into_slice_opts(self) -> (CommandArg, Option<CommandArg>, SliceOption) {
        (self.0 .0.into(), None, self.0 .1)
    }
}

impl<S, E> SliceArg for Args<(S, E, SliceOption)>
where
    S: Into<CommandArg>,
    E: Into<CommandArg>,
{
    fn into_slice_opts(self) -> (CommandArg, Option<CommandArg>, SliceOption) {
        (self.0 .0.into(), Some(self.0 .1.into()), self.0 .2)
    }
}
