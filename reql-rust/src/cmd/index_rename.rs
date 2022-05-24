use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;

use crate::Command;
use crate::types::IndexResponseType;

use super::run;

pub struct IndexRenameBuilder(Command, IndexRenameOption);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

impl IndexRenameBuilder {
    pub fn new(old_index_name: &str, new_index_name: &str) -> Self {
        let arg_1 = Command::from_json(old_index_name);
        let arg_2 = Command::from_json(new_index_name);
        let command = Command::new(TermType::IndexRename).with_arg(arg_1).with_arg(arg_2);
        
        IndexRenameBuilder(command, Default::default())
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<IndexResponseType>> {        
        self.0.with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, IndexResponseType>(arg)
            .try_next()
            .await
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.1.overwrite = Some(overwrite);
        self
    }

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
