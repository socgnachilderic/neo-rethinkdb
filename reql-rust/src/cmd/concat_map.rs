use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsSequence};
use crate::{Command, Func};

#[derive(Debug, Clone)]
pub struct ConcatMapBuilder<A>(pub(crate) Command, pub(crate) PhantomData<A>);

impl<A: Unpin + DeserializeOwned> ConcatMapBuilder<A> {
    pub(crate) fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::ConcatMap).with_arg(func);
        
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

impl<A: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<A> for ConcatMapBuilder<A> { }
impl<A> ReqlOpsArray for ConcatMapBuilder<A> { }

impl<A> SuperOps for ConcatMapBuilder<A> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
