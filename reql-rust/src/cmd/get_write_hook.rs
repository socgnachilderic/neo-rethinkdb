use crate::{ops::ReqlOps, types::WriteHookResponseType, Command};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

#[derive(Debug, Clone)]
pub struct GetWriteBuilder(pub(crate) Command);

impl GetWriteBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::GetWriteHook);

        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Option<WriteHookResponseType>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Option<WriteHookResponseType>>> {
        self.get_parent()
            .run::<_, Option<WriteHookResponseType>>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for GetWriteBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for GetWriteBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
