use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::ops::ReqlOps;
use crate::types::StatusResponseType;
use crate::Command;

#[derive(Debug, Clone)]
pub struct StatusBuilder(pub(crate) Command);

impl StatusBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Status);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<StatusResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<StatusResponseType>> {
        self.get_parent().run::<_, StatusResponseType>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for StatusBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for StatusBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
