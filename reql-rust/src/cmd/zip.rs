use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::ops::ReqlOps;
use crate::Command;

pub struct ZipBuilder(pub(crate) Command);

impl ZipBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Zip);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<serde_json::Value>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<serde_json::Value>> {
        self.get_parent().run::<_, serde_json::Value>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for ZipBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for ZipBuilder {
    fn into(self) -> Command {
        self.0
    }
}
