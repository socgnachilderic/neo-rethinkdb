use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Command, Func};
use crate::ops::{ReqlOpsSequence, SuperOps, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct FoldBuilder<A, B>(
    pub(crate) Command,
    // pub(crate) FoldOption,
    pub(crate) PhantomData<A>,
    pub(crate) PhantomData<B>,
);

// #[derive(Debug, Clone, Serialize, Default)]
// #[non_exhaustive]
// pub(crate) struct FoldOption {
//     emit: Option<Command>,
//     final_emit: Option<Command>,
// }

impl<A, B> FoldBuilder<A, B>
where
    A: Serialize,
    B: Unpin + Serialize + DeserializeOwned,
{
    pub(crate) fn new(base: A, func: Func) -> Self {
        let arg = Command::from_json(base);
        let Func(func) = func;
        let command = Command::new(TermType::Fold)
            .with_arg(arg)
            .with_arg(func);

        Self(command, PhantomData, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<B>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<B>> {
        self.0.into_arg::<()>().into_cmd().run::<_, B>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<A, B: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<B> for FoldBuilder<A, B> {}

impl<A, B> ReqlOpsDocManipulation for FoldBuilder<A, B> { }

impl<A, B> SuperOps for FoldBuilder<A, B> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
