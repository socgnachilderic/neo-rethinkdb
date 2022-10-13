use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new<S, T>(values: T) -> Command
where
    S: Serialize,
    T: IntoIterator<Item = S>,
{
    values
        .into_iter()
        .map(Command::from_json)
        .fold(Command::new(TermType::Object), |cmd, value| {
            cmd.with_arg(value)
        })
}
