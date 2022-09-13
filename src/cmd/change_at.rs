use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(offset: isize, value: impl Serialize) -> Command {
    let arg_offset = Command::from_json(offset);
    let arg_value = Command::from_json(value);

    Command::new(TermType::ChangeAt)
        .with_arg(arg_offset)
        .with_arg(arg_value)
}

// TODO write test
