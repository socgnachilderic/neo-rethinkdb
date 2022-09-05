use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use crate::ops::{ReqlOps, ReqlOpsArray};
use crate::types::Interleave;
use crate::{Command, Result};

#[derive(Debug, Clone)]
pub struct UnionBuilder<T>(
    pub(crate) Command,
    pub(crate) UnionOption,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct UnionOption {
    pub interleave: Option<Interleave>,
}

impl<T: Unpin + Serialize + DeserializeOwned> UnionBuilder<T> {
    pub(crate) fn new(values: &[&impl ReqlOps]) -> Self {
        let mut command = Command::new(TermType::Union);

        for val in values {
            command = command.with_arg(val.get_parent());
        }

        Self(command, UnionOption::default(), PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = Result<T>> {
        self.get_parent().run::<_, T>(arg)
    }

    pub fn with_interleave(mut self, interleave: Interleave) -> Self {
        self.1.interleave = Some(interleave);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsArray for UnionBuilder<T> {}

impl<T> ReqlOps for UnionBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0
            .clone()
            .with_opts(&self.1)
            .into_arg::<()>()
            .into_cmd()
    }
}

impl<T> Into<Command> for UnionBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
