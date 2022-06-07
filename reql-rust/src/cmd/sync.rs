use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::Command;
use crate::types::SyncResponseType;

#[derive(Debug, Clone)]
pub struct SyncBuilder(pub(crate) Command);

impl SyncBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Sync);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<SyncResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<SyncResponseType>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, SyncResponseType>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
