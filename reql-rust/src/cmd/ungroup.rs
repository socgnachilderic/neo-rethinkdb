use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Command, Result};
use crate::ops::{ReqlOpsSequence, ReqlOps, ReqlOpsDocManipulation};
use crate::types::{UngroupResponseType, Sequence};

#[derive(Debug, Clone)]
pub struct UngroupBuilder<G, V>(
    pub(crate) Command,
    pub(crate) PhantomData<G>,
    pub(crate) PhantomData<V>,
);

impl<G, V> UngroupBuilder<G, V>
where
    G: Unpin + Serialize + DeserializeOwned,
    V: Unpin + Serialize + DeserializeOwned,
{
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Ungroup);

        Self(command, PhantomData, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> Result<Option<Sequence<UngroupResponseType<G, V>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = Result<Sequence<UngroupResponseType<G, V>>>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, Sequence<UngroupResponseType<G, V>>>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<G, V> ReqlOpsSequence<UngroupResponseType<G, V>> for UngroupBuilder<G, V>
where
    G: Unpin + Serialize + DeserializeOwned,
    V: Unpin + Serialize + DeserializeOwned,
{}

impl<G, V> ReqlOpsDocManipulation for UngroupBuilder<G, V> { }

impl<G, V> ReqlOps for UngroupBuilder<G, V> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
