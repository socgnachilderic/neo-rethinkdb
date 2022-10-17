use crate::Command;
use ql2::term::TermType;

pub(crate) fn new() -> Command {
    Command::new(TermType::TableList)
}
