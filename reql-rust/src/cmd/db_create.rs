use futures::TryStreamExt;
use ql2::term::TermType;

use crate::Command;
use crate::types::DbResponseType;

use super::run;

pub struct DbCreateBuilder(Command);

impl DbCreateBuilder {
    pub fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        DbCreateBuilder(args)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DbResponseType>> {        
        Command::new(TermType::DbCreate)
            .with_arg(self.0)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbResponseType>(arg)
            .try_next().await
    }
}
