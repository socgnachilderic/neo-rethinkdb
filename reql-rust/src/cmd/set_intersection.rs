use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};

use crate::Command;
use crate::ops::{ReqlOps, ReqlOpsSequence, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct SetIntersectionBuilder<T>(
    pub(crate) Command,
    pub(crate) PhantomData<T>
);

impl<T: Unpin + Serialize + DeserializeOwned> SetIntersectionBuilder<T> {
    pub(crate) fn new(values: &[impl Serialize]) -> Self {
        let arg = Command::from_json(values);
        let command = Command::new(TermType::SetIntersection).with_arg(arg);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<T>> {        
        self.0.into_arg::<()>().into_cmd().run::<_, T>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for SetIntersectionBuilder<T> { }
impl<T> ReqlOpsDocManipulation for SetIntersectionBuilder<T> { }

impl<T> ReqlOps for SetIntersectionBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
