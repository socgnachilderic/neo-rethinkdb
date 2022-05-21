use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::DbDropReturnType};

use super::run;

pub struct DbDropBuilder(Command);

impl DbDropBuilder {
    pub fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        DbDropBuilder(args)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<DbDropReturnType>> {
        Command::new(TermType::DbDrop)
            .with_arg(self.0)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbDropReturnType>(arg)
    }
}

