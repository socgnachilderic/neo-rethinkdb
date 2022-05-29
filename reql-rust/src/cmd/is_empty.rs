use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::ops::SuperOps;
use crate::Command;

#[derive(Debug, Clone)]
pub struct IsEmptyBuilder(pub(crate) Command);

impl IsEmptyBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::IsEmpty);
        
        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<bool>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<bool>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, bool>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl SuperOps for IsEmptyBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}