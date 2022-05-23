use crate::{types::WriteHookResponseType, Command};
use futures::TryStreamExt;
use ql2::term::TermType;
pub struct GetWriteBuilder(Command);

impl GetWriteBuilder {
    pub fn new() -> Self {
        let command = Command::new(TermType::GetWriteHook);

        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Option<WriteHookResponseType>>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Option<WriteHookResponseType>>(arg)
            .try_next().await
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
