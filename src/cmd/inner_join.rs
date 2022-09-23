use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(other_table: Command, func: Func) -> Command {
    let Func(func) = func;

    Command::new(TermType::InnerJoin)
        .with_arg(other_table)
        .with_arg(func)
}

// TODO write test
