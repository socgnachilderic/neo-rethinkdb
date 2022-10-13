use ql2::term::TermType;

use crate::arguments::Permission;
use crate::Command;

pub(crate) fn new(username: impl Into<String>, permission: Permission) -> Command {
    Command::new(TermType::Grant)
        .with_arg(Command::from_json(username.into()))
        .with_arg(Command::from_json(permission))
}
