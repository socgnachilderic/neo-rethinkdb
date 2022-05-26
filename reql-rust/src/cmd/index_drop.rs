use crate::types::IndexResponseType;
use crate::Command;
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

#[derive(Debug, Clone)]
pub struct IndexDropBuilder(pub(crate) Command);

impl IndexDropBuilder {
    pub(crate) fn new(index_name: &str) -> Self {
        let args = Command::from_json(index_name);
        let command = Command::new(TermType::IndexDrop).with_arg(args);

        IndexDropBuilder(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<IndexResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<IndexResponseType>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, IndexResponseType>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
