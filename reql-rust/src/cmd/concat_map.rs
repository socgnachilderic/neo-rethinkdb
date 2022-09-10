use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(func: Func) -> Command {
    Command::new(TermType::ConcatMap).with_arg(func.0)
}

// TODO write test
