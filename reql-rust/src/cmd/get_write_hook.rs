use crate::{types::WriteHookResponseType, Command};
use futures::Stream;
use ql2::term::TermType;
pub struct GetWriteBuilder(Command);

impl GetWriteBuilder {
    pub fn new() -> Self {
        let command = Command::new(TermType::GetWriteHook);

        Self(command)
    }

    pub fn run(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Option<WriteHookResponseType>>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Option<WriteHookResponseType>>(arg)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
