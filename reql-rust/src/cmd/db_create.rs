use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::DbCreateReturnType};

use super::run;

pub struct DbCreateBuilder(Command);

impl DbCreateBuilder {
    pub fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        DbCreateBuilder(args)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<DbCreateReturnType>> {        
        Command::new(TermType::DbCreate)
            .with_arg(self.0)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbCreateReturnType>(arg)
    }
}
