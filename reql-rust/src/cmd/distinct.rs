use std::borrow::Cow;
use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ops::{ReqlOpsSequence, SuperOps, ReqlOpsDocManipulation};
use crate::Command;

use super::StaticString;

#[derive(Debug, Clone)]
pub struct DistinctBuilder<T>(pub(crate) Command, pub(crate) PhantomData<T>);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct DistinctOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + Serialize + DeserializeOwned> DistinctBuilder<T> {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Distinct);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<T>> {
        self.0.into_arg::<()>().into_cmd().run::<_, T>(arg)
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        let index = Some(index.static_string());
        let index = DistinctOption { index };
        
        self.0 = self.0.with_opts(index);

        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for DistinctBuilder<T> {}

impl<T> ReqlOpsDocManipulation for DistinctBuilder<T> { }

impl<T> SuperOps for DistinctBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
