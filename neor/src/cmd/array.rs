use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new<S, T>(values: T) -> Command
where
    S: Into<CommandArg>,
    T: IntoIterator<Item = S>,
{
    values
        .into_iter()
        .map(|cmd| cmd.into().to_cmd())
        .fold(Command::new(TermType::MakeArray), |cmd, value| {
            cmd.with_arg(value)
        })
}
