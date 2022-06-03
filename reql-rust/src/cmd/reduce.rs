use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::SuperOps;
use crate::{Command, Func};

#[derive(Debug, Clone)]
pub struct ReduceBuilder<A>(pub(crate) Command, pub(crate) PhantomData<A>);

impl<A: Unpin + DeserializeOwned> ReduceBuilder<A> {
    pub(crate) fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Reduce).with_arg(func);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<A>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<A>> {
        self.0.into_arg::<()>().into_cmd().run::<_, A>(arg)
    }

    pub fn with_sequences(mut self, sequences: &[impl Serialize]) -> Self {
        for seq in sequences {
            let arg = Command::from_json(seq);
            self.0 = self.0.with_arg(arg)
        }

        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<A> SuperOps for ReduceBuilder<A> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
