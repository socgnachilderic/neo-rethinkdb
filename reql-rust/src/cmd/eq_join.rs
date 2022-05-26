use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::{types::JoinResponseType, Command, Func};

use super::{run, table::TableBuilder, JoinOps, DocManipulationOps, SuperOps};

#[derive(Debug, Clone)]
pub struct EqJoinBuilder<A, T>(
    pub(crate) Command,
    pub(crate) EqJoinOption,
    pub(crate) Option<A>,
    pub(crate) Option<T>,
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

        Self(command, EqJoinOption::default(), None, None)
    }

    pub(crate) fn new_by_func(func: Func, right_table: &TableBuilder<A>) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::EqJoin)
            .with_arg(func)
            .with_arg(right_table.0.clone());

        Self(command, EqJoinOption::default(), None, None)
    }

    pub async fn run(
        self,
        arg: impl run::Arg,
    ) -> crate::Result<Option<Vec<JoinResponseType<T, A>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl run::Arg,
    ) -> impl Stream<Item = crate::Result<Vec<JoinResponseType<T, A>>>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<JoinResponseType<T, A>>>(arg)
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

impl<A, T> JoinOps for EqJoinBuilder<A, T> {}

impl<A, T> DocManipulationOps for EqJoinBuilder<A, T> {}

impl<A, T> SuperOps for EqJoinBuilder<A, T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
