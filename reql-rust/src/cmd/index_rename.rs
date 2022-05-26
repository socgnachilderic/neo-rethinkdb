use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::types::IndexResponseType;
use crate::Command;

#[derive(Debug, Clone)]
pub struct IndexRenameBuilder(pub(crate) Command, pub(crate) IndexRenameOption);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub(crate) struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

impl IndexRenameBuilder {
    pub(crate) fn new(old_index_name: &str, new_index_name: &str) -> Self {
        let arg_1 = Command::from_json(old_index_name);
        let arg_2 = Command::from_json(new_index_name);
        let command = Command::new(TermType::IndexRename)
            .with_arg(arg_1)
            .with_arg(arg_2);

        IndexRenameBuilder(command, Default::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<IndexResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<IndexResponseType>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, IndexResponseType>(arg)
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.1.overwrite = Some(overwrite);
        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
