use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::{types::WriteHookResponseType, Command, Func};

#[derive(Debug, Clone)]
pub struct SetWriteHookBuilder(pub(crate) Command);

impl SetWriteHookBuilder {
    pub(crate) fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::SetWriteHook).with_arg(func);

        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<WriteHookResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<WriteHookResponseType>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, WriteHookResponseType>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
