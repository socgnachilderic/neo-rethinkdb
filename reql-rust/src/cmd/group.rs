use std::{borrow::Cow, marker::PhantomData};

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::{ReqlOpsGroupedStream, SuperOps};
use crate::types::GroupStream;
use crate::{Command, Func, Result};

use super::StaticString;

#[derive(Debug, Clone)]
pub struct GroupBuilder<G, V>(
    pub(crate) Command,
    pub(crate) GroupOption,
    pub(crate) PhantomData<G>,
    pub(crate) PhantomData<V>,
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct GroupOption {
    pub index: Option<Cow<'static, str>>,
    pub multi: Option<bool>,
}

impl<G, V> GroupBuilder<G, V>
where
    G: Unpin + Serialize + DeserializeOwned,
    V: Unpin + Serialize + DeserializeOwned,
{
    pub(crate) fn new(fields: &[&str]) -> Self {
        let mut command = Command::new(TermType::Group);
        
        for field in fields {
            let arg = Command::from_json(field);
            command = command.with_arg(arg);
        }

        Self(command, GroupOption::default(), PhantomData, PhantomData)
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Group).with_arg(func);

        Self(command, GroupOption::default(), PhantomData, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> Result<Option<GroupStream<G, V>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = Result<GroupStream<G, V>>> {
        let mut command = self.0;

        if self.1.index.is_some() || self.1.multi.is_some() {
            command = command.with_opts(self.1);
        }

        command.into_arg::<()>()
            .into_cmd()
            .run::<_, GroupStream<G, V>>(arg)
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.1.index = Some(index.static_string());
        self
    }

    pub fn with_multi(mut self, multi: bool) -> Self {
        self.1.multi = Some(multi);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<G, V> ReqlOpsGroupedStream for GroupBuilder<G, V>
where
    G: Serialize,
    V: Unpin + Serialize + DeserializeOwned,
{}

impl<G, V> SuperOps for GroupBuilder<G, V> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
