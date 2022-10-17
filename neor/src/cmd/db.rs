use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(db_name: impl Into<String>) -> Command {
    let arg = Command::from_json(db_name.into());

    Command::new(TermType::Db).with_arg(arg)
}
