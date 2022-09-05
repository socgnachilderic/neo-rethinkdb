use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::{ReqlOps, ReqlOpsDocManipulation, ReqlOpsJoin, ReqlOpsSequence};
use crate::types::{Document, JoinResponseType, Sequence};
use crate::{Command, Func};

use super::{run, table::TableBuilder};

#[derive(Debug, Clone)]
pub struct EqJoinBuilder<A, T>(
    pub(crate) Command,
    pub(crate) EqJoinOption,
    pub(crate) PhantomData<A>,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub(crate) struct EqJoinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

impl<A, T> EqJoinBuilder<A, T>
where
    A: Unpin + Serialize + DeserializeOwned,
    T: Unpin + Serialize + DeserializeOwned,
{
    pub(crate) fn new(left_field: &str, right_table: &TableBuilder<A>) -> Self {
        let command = Command::new(TermType::EqJoin)
            .with_arg(Command::from_json(left_field))
            .with_arg(right_table.0.clone());

        Self(command, EqJoinOption::default(), PhantomData, PhantomData)
    }

    pub(crate) fn new_by_func(func: Func, right_table: &TableBuilder<A>) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::EqJoin)
            .with_arg(func)
            .with_arg(right_table.0.clone());

        Self(command, EqJoinOption::default(), PhantomData, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl run::Arg,
    ) -> crate::Result<Option<Sequence<JoinResponseType<T, Document<A>>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl run::Arg,
    ) -> impl Stream<Item = crate::Result<Sequence<JoinResponseType<T, Document<A>>>>> {
        self.get_parent()
            .run::<_, Sequence<JoinResponseType<T, Document<A>>>>(arg)
    }

    pub fn with_ordered(mut self, ordered: bool) -> Self {
        self.1.ordered = Some(ordered);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<A, T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for EqJoinBuilder<A, T> {}
impl<A, T: Unpin + Serialize + DeserializeOwned> ReqlOpsJoin<T> for EqJoinBuilder<A, T> {}
impl<A, T> ReqlOpsDocManipulation for EqJoinBuilder<A, T> {}

impl<A, T> ReqlOps for EqJoinBuilder<A, T> {
    fn get_parent(&self) -> Command {
        self.0.clone().with_opts(&self.1).into_arg::<()>().into_cmd()
    }
}

impl<A, T> Into<Command> for EqJoinBuilder<A, T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
