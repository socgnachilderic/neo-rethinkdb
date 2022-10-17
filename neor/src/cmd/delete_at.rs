use ql2::term::TermType;

use crate::arguments::Args;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl DeleteAtArg) -> Command {
    let (start_offset, end_offset) = args.into_delete_at_opts();
    let mut command = start_offset.add_to_cmd(TermType::DeleteAt);

    if let Some(end_offset) = end_offset {
        command = command.with_arg(end_offset.to_cmd());
    }

    command
}

pub trait DeleteAtArg {
    fn into_delete_at_opts(self) -> (CommandArg, Option<CommandArg>);
}

impl<T> DeleteAtArg for T
where
    T: Into<CommandArg>,
{
    fn into_delete_at_opts(self) -> (CommandArg, Option<CommandArg>) {
        (self.into(), None)
    }
}

impl<S, E> DeleteAtArg for Args<(S, E)>
where
    S: Into<CommandArg>,
    E: Into<CommandArg>,
{
    fn into_delete_at_opts(self) -> (CommandArg, Option<CommandArg>) {
        (self.0 .0.into(), Some(self.0 .1.into()))
    }
}
