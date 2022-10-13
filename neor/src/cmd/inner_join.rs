use ql2::term::TermType;

use crate::{Command, Func};

pub(crate) fn new(other_table: Command, func: Func) -> Command {
    Command::new(TermType::InnerJoin)
        .with_arg(other_table)
        .with_arg(func.0)
}
