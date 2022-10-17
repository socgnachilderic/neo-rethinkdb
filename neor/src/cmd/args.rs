use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new<T, S>(values: T) -> Command
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    Command::new(TermType::Args).with_arg(Command::from_json(values))
}
