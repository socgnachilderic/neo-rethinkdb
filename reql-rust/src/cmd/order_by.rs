use std::borrow::Cow;
use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsSequence};
use crate::{Command, Func};
use crate::types::{Document, Sequence};

use super::StaticString;

#[derive(Debug, Clone)]
pub struct OrderByBuilder<T>(
    pub(crate) Command,
    pub(crate) OrderByOption,
    pub(crate) PhantomData<T>
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct OrderByOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + DeserializeOwned> OrderByBuilder<T> {
    pub(crate) fn new() -> Self {
        Self::constructor(None)
    }

    pub(crate) fn new_by_key(key: &str) -> Self {
        let arg = Command::from_json(key);
        Self::constructor(Some(arg))
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        Self::constructor(Some(func))
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Sequence<Document<T>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Sequence<Document<T>>>> {
        let mut command = self.0;
        
        if self.1.index.is_some() {
            command = command.with_opts(self.1);
        }

        command.into_arg::<()>()
            .into_cmd()
            .run::<_, Sequence<Document<T>>>(arg)
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.1.index = Some(index.static_string());
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }

    fn constructor(arg: Option<Command>) -> Self {
        let mut command = Command::new(TermType::OrderBy);

        if let Some(arg) = arg {
            command = command.with_arg(arg);
        }
        
        Self(command, OrderByOption::default(), PhantomData)
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for OrderByBuilder<T> { }
impl<T> ReqlOpsArray for OrderByBuilder<T> { }

impl<T> SuperOps for OrderByBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
