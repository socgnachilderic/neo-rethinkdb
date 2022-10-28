use ql2::term::TermType;

use crate::command_tools::CommandArg;
use crate::Command;

pub(crate) fn new<S, T>(values: T) -> Command
where
    S: Into<CommandArg>,
    T: IntoIterator<Item = S>,
{
    values
        .into_iter()
        .fold(Command::new(TermType::Object), |cmd, value| {
            cmd.with_arg(value.into().to_cmd())
        })
}
