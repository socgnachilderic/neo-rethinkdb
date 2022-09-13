use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(values: Vec<impl Serialize>) -> Command {
    let arg = Command::from_json(values);

    Command::new(TermType::SetDifference).with_arg(arg)
}

// TODO write test
