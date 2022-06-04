use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsSequence, ReqlOpsDocManipulation};
use crate::Command;

#[derive(Debug, Clone)]
pub struct WithFieldsBuilder<A>(pub(crate) Command, pub(crate) PhantomData<A>);

impl<A: Unpin + DeserializeOwned> WithFieldsBuilder<A> {
    pub(crate) fn new(fields: &[&str]) -> Self {
        let arg = Command::from_json(fields);
        let command = Command::new(TermType::WithFields).with_arg(arg);
        
        Self(command, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<A>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<A>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, A>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<A: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<A> for WithFieldsBuilder<A> { }
impl<A> ReqlOpsArray for WithFieldsBuilder<A> { }
impl<T> ReqlOpsDocManipulation for WithFieldsBuilder<T> { }

impl<A> SuperOps for WithFieldsBuilder<A> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
