use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(arg: Func) -> Command {
    Command::new(TermType::ForEach).with_arg(arg.0)
}

// TODO write test
