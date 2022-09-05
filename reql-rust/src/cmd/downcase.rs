use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::ops::ReqlOps;
use crate::Command;

#[derive(Debug, Clone)]
pub struct DowncaseBuilder(pub(crate) Command);

impl DowncaseBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Downcase);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Cow<'static, str>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Cow<'static, str>>> {
        self.get_parent().run::<_, Cow<'static, str>>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for DowncaseBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for DowncaseBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
