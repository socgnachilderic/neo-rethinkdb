use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::{ReqlOps, ReqlOpsArray, ReqlOpsDocManipulation, ReqlOpsSequence};
use crate::{Command, Func};

#[derive(Debug, Clone)]
pub struct MapBuilder<A>(pub(crate) Command, pub(crate) PhantomData<A>);

impl<A: Unpin + DeserializeOwned> MapBuilder<A> {
    pub(crate) fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Map).with_arg(func);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<A>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<A>> {
        self.get_parent().run::<_, A>(arg)
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

impl<A: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<A> for MapBuilder<A> {}
impl<A> ReqlOpsArray for MapBuilder<A> {}
impl<T> ReqlOpsDocManipulation for MapBuilder<T> {}

impl<A> ReqlOps for MapBuilder<A> {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for MapBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
