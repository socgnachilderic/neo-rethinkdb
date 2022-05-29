use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsSequence};
use crate::{Command, Func};
use crate::types::{Document, Sequence};

#[derive(Debug, Clone)]
pub struct OffsetsOfBuilder<T>(pub(crate) Command, pub(crate) PhantomData<T>);

impl<T: Unpin + DeserializeOwned> OffsetsOfBuilder<T> {
    pub(crate) fn new(datum: impl Serialize) -> Self {
        let arg = Command::from_json(datum);
        Self::constructor(arg)
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        Self::constructor(func)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Sequence<Document<T>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Sequence<Document<T>>>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, Sequence<Document<T>>>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }

    fn constructor(command: Command) -> Self {
        let command = Command::new(TermType::OffsetsOf).with_arg(command);
        
        Self(command, PhantomData)
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for OffsetsOfBuilder<T> { }
impl<T> ReqlOpsArray for OffsetsOfBuilder<T> { }

impl<T> SuperOps for OffsetsOfBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
