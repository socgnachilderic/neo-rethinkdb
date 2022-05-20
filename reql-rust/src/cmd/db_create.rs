use std::borrow::Cow;

use crate::{Command};
use ql2::term::TermType;

use super::StaticString;

pub(crate) struct DbCreate(Cow<'static, str>);

impl DbCreate {
    pub fn new(db_name: &'static str) -> Command {
        let db: Command = DbCreate(db_name.static_string()).into();
        
        Command::new(TermType::DbCreate)
            .with_arg(db)
            .into_arg::<()>()
            .into_cmd()
    }
}

impl Into<Command> for DbCreate {
    fn into(self) -> Command {
        Command::from_json(self.0)
    }
}
