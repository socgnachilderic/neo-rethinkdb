use crate::Command;
use crate::types::IndexResponseType;
use futures::TryStreamExt;
use ql2::term::TermType;

use super::run;

pub struct IndexDropBuilder(Command);

impl IndexDropBuilder {
    pub fn new(index_name: &str) -> Self {
        let args = Command::from_json(index_name);
        let command = Command::new(TermType::IndexDrop).with_arg(args);

        IndexDropBuilder(command)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<IndexResponseType>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, IndexResponseType>(arg)
            .try_next().await
    }
}
