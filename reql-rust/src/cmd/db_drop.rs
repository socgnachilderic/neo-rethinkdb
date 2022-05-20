use std::borrow::Cow;

use crate::{Command};
use ql2::term::TermType;

use super::StaticString;

pub(crate) struct DbDrop(Cow<'static, str>);

impl DbDrop {
    pub fn new(db_name: &'static str) -> Command {
        let db: Command = DbDrop(db_name.static_string()).into();
        
        Command::new(TermType::DbDrop)
            .with_arg(db)
            .into_arg::<()>()
            .into_cmd()
    }
}

impl Into<Command> for DbDrop {
    fn into(self) -> Command {
        Command::from_json(self.0)
    }
}
