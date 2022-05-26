use crate::types::IndexStatusResponseType;
use crate::Command;
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use super::run;

#[derive(Debug, Clone)]
pub struct IndexWaitBuilder(Command);

impl IndexWaitBuilder {
    /// Get all indexes of table
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::IndexWait);

        IndexWaitBuilder(command)
    }

    pub async fn run(
        self,
        arg: impl run::Arg,
    ) -> crate::Result<Option<Vec<IndexStatusResponseType>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl run::Arg,
    ) -> impl Stream<Item = crate::Result<Vec<IndexStatusResponseType>>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<IndexStatusResponseType>>(arg)
    }

    /// Get one index of table
    pub fn with_one_index(mut self, index_name: &str) -> Self {
        let args = Command::from_json(index_name);

        self.0 = self.0.with_arg(args);
        self
    }

    /// Get an index array of table
    pub fn with_indexes(mut self, index_names: &[&str]) -> Self {
        for index_name in index_names {
            let args = Command::from_json(index_name);
            self.0 = self.0.with_arg(args);
        }

        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
