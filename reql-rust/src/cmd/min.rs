use std::{borrow::Cow, marker::PhantomData};

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};

use crate::{Command, Func};

use super::StaticString;

#[derive(Debug, Clone)]
pub struct MinBuilder<T>(
    pub(crate) Command,
    pub(crate) MinOption,
    pub(crate) PhantomData<T>
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct MinOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + Serialize + DeserializeOwned> MinBuilder<T> {
    pub(crate) fn new() -> Self {
        Self::constructor(None)
    }

    pub(crate) fn new_by_value(field_name: &str) -> Self {
        let arg = Command::from_json(field_name);
        Self::constructor(Some(arg))
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        Self::constructor(Some(func))
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<T>> {
        let mut command = self.0;

        if self.1.index.is_some() {
            command = command.with_opts(self.1);
        }
        
        command.into_arg::<()>().into_cmd().run::<_, T>(arg)
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
        let mut command = Command::new(TermType::Min);

        if let Some(arg) = arg {
            command = command.with_arg(arg)
        }

        Self(command, MinOption::default(), PhantomData)
    }
}
