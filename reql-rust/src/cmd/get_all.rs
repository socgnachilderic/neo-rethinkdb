use std::{borrow::Cow, marker::PhantomData};

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::{ReqlOps, ReqlOpsDocManipulation, ReqlOpsSequence};
use crate::{Command, Result};

use super::StaticString;

#[derive(Debug, Clone)]
pub struct GetAllBuilder<T>(
    pub(crate) Command,
    pub(crate) GetAllOption,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct GetAllOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + Serialize + DeserializeOwned> GetAllBuilder<T> {
    pub(crate) fn new(values: &[impl Serialize]) -> Self {
        assert!(values.len() > 0);

        let mut command = Command::new(TermType::GetAll);

        for val in values {
            let arg = Command::from_json(val);
            command = command.with_arg(arg);
        }

        Self(command, GetAllOption::default(), PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = Result<T>> {
        self.get_parent().run::<_, T>(arg)
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.1.index = Some(index.static_string());
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for GetAllBuilder<T> {}

impl<T> ReqlOpsDocManipulation for GetAllBuilder<T> {}

impl<T> ReqlOps for GetAllBuilder<T> {
    fn get_parent(&self) -> Command {
        let mut command = self.0.clone();

        if self.1.index.is_some() {
            command = command.with_opts(&self.1);
        }

        command.into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for GetAllBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
