use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::types::DbResponseType;
use crate::Command;

#[derive(Debug, Clone)]
pub struct DbDropBuilder(pub(crate) Command);

impl DbDropBuilder {
    pub(crate) fn new(db_name: &str) -> Self {
        let args = Command::from_json(db_name);
        let command = Command::new(TermType::DbDrop).with_arg(args);
        DbDropBuilder(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<DbResponseType>> {
        self.make_query(arg).try_next().await
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
