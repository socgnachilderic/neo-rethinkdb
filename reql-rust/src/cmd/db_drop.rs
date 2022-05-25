use futures::TryStreamExt;
use ql2::term::TermType;

use crate::Command;
use crate::types::DbResponseType;

use super::run;

pub struct DbDropBuilder(Command);

impl DbDropBuilder {
    pub fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        DbDropBuilder(args)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DbResponseType>> {
        Command::new(TermType::DbDrop)
            .with_arg(self.0)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbResponseType>(arg)
            .try_next().await
    }
}

