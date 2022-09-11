use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(value: impl Serialize) -> Command {
    let arg = Command::from_json(value);

    Command::new(TermType::Append).with_arg(arg)
}

// TODO write test
