use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cmd::run;
use crate::ops::{ReqlOps, ReqlOpsDocManipulation, ReqlOpsGeometry, ReqlOpsSequence};
use crate::Command;

#[derive(Debug, Clone)]
pub struct IncludesBuilder<T>(pub(crate) Command, PhantomData<T>);

impl<T: Unpin + Serialize + DeserializeOwned> IncludesBuilder<T> {
    pub(crate) fn new<A: ReqlOpsGeometry + Serialize>(geometry: A) -> Self {
        let arg = Command::from_json(geometry);
        let command = Command::new(TermType::Includes).with_arg(arg);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<T>> {
        self.get_parent().run::<_, T>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for IncludesBuilder<T> {}

impl<T> ReqlOpsDocManipulation for IncludesBuilder<T> {}

impl<T> ReqlOps for IncludesBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for IncludesBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
