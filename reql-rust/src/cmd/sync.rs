use crate::{types::SyncResponseType, Command};
use futures::TryStreamExt;
use ql2::term::TermType;
pub struct SyncBuilder(Command);

impl SyncBuilder {
    pub fn new() -> Self {
        let command = Command::new(TermType::Sync);

        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<SyncResponseType>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, SyncResponseType>(arg)
            .try_next().await
    }

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

