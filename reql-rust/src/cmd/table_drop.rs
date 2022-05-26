use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::types::DbResponseType;
use crate::Command;

#[derive(Debug, Clone)]
pub struct TableDropBuilder(pub(crate) Command);

impl TableDropBuilder {
    pub(crate) fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::TableDrop).with_arg(args);

        TableDropBuilder(command)
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

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
