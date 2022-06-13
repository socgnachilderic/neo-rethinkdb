use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::Command;
use crate::ops::ReqlOps;
use crate::types::{ConfigChangeValue, WritingResponseType};

#[derive(Debug, Clone)]
pub struct ConfigBuilder(pub(crate) Command);

impl ConfigBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Config);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<ConfigChangeValue>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<ConfigChangeValue>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, ConfigChangeValue>(arg)
    }

    pub fn update(&self, configs: impl Serialize) -> super::update::UpdateBuilder<WritingResponseType<()>> {
        super::update::UpdateBuilder::new(configs)._with_parent(self.get_parent())
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for ConfigBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
