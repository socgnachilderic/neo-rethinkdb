use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(other_table: Command, func: Func) -> Command {
    let Func(func) = func;

    Command::new(TermType::OuterJoin)
        .with_arg(other_table)
        .with_arg(func)
}
