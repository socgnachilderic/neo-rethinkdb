use std::borrow::Cow;
use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::ops::{ReqlOps, ReqlOpsGeometry};
use crate::Command;

use super::StaticString;

#[derive(Debug, Clone)]
pub struct GetIntersectingBuilder<A>(pub(crate) Command, pub(crate) PhantomData<A>);

#[derive(Debug, Clone, Serialize, Default)]
#[non_exhaustive]
pub struct GetIntersectingOption {
    pub index: Cow<'static, str>,
}

impl<A: Serialize + ReqlOpsGeometry> GetIntersectingBuilder<A> {
    pub(crate) fn new(geometry: &A, index: &'static str) -> Self {
        let arg = Command::from_json(geometry);
        let opts = GetIntersectingOption {
            index: index.static_string(),
        };
        let command = Command::new(TermType::GetIntersecting)
            .with_arg(arg)
            .with_opts(opts);

        Self(command, PhantomData)
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

impl<T> ReqlOpsGeometry for GetIntersectingBuilder<T> {}

impl<T> ReqlOps for GetIntersectingBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for GetIntersectingBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
