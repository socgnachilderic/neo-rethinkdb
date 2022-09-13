use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(func: Func) -> Command {
    Command::new(TermType::ConcatMap).with_arg(func.0)
}

// TODO write test
