use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(db_name: impl Into<String>) -> Command {
    Command::new(TermType::DbDrop).with_arg(Command::from_json(db_name.into()))
}
