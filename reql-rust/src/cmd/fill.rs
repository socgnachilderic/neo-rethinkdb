use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::Command;
use crate::ops::{ReqlOpsGeometry, ReqlOps};
use crate::types::Polygon;

#[derive(Debug, Clone)]
pub struct FillBuilder(pub(crate) Command);

impl FillBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Fill);

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Polygon>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Polygon>> {
        self.0.into_arg::<()>().into_cmd().run::<_, Polygon>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOpsGeometry for FillBuilder {}

impl ReqlOps for FillBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
