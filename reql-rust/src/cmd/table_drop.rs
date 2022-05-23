use futures::TryStreamExt;
use ql2::term::TermType;

use crate::Command;
use crate::types::DbResponseType;

use super::run;

pub struct TableDropBuilder(Command);

impl TableDropBuilder {
    pub fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::TableDrop)
            .with_arg(args);

        TableDropBuilder(command)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DbResponseType>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, DbResponseType>(arg)
            .try_next().await
    }
}
