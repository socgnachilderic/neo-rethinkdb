use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(arg: Func) -> Command {
    Command::new(TermType::ForEach).with_arg(arg.0)
}

// TODO write test
