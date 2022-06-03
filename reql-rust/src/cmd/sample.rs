use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsSequence};
use crate::Command;

#[derive(Debug, Clone)]
pub struct SampleBuilder<T>(pub(crate) Command, pub(crate) PhantomData<T>);

impl<T: Unpin + DeserializeOwned> SampleBuilder<T> {
    pub(crate) fn new(number: usize) -> Self {
        let arg = Command::from_json(number);
        let command = Command::new(TermType::Sample).with_arg(arg);
        
        Self(command, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<T>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, T>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for SampleBuilder<T> { }
impl<T> ReqlOpsArray for SampleBuilder<T> { }

impl<T> SuperOps for SampleBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
