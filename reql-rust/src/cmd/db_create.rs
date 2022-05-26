use futures::{TryStreamExt, Stream};
use ql2::term::TermType;

use crate::Command;
use crate::types::DbResponseType;

use super::run;

#[derive(Debug, Clone)]
pub struct DbCreateBuilder(pub(crate) Command);

impl DbCreateBuilder {
    pub(crate) fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        let command =  Command::new(TermType::DbCreate).with_arg(args);
        DbCreateBuilder(command)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DbResponseType>> {        
        self.make_query(arg)
            .try_next()
            .await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<DbResponseType>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbResponseType>(arg)
    }
}
