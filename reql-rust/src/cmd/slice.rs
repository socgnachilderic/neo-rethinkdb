use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ops::{ReqlOps, ReqlOpsArray, ReqlOpsDocManipulation, ReqlOpsSequence};
use crate::types::Status;
use crate::Command;

#[derive(Debug, Clone)]
pub struct SliceBuilder<T>(
    pub(crate) Command,
    pub(crate) SliceOption,
    pub(crate) PhantomData<T>,
);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct SliceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

impl<T: Unpin + DeserializeOwned> SliceBuilder<T> {
    pub(crate) fn new(start_offset: usize, end_offset: Option<usize>) -> Self {
        let start_offset = Command::from_json(start_offset);
        let mut command = Command::new(TermType::Slice).with_arg(start_offset);

        if let Some(start_offset) = end_offset {
            let start_offset = Command::from_json(start_offset);
            command = command.with_arg(start_offset);
        }

        Self(command, SliceOption::default(), PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<T>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<T>> {
        self.get_parent().run::<_, T>(arg)
    }

    pub fn with_left_bound(mut self, status: Status) -> Self {
        self.1.left_bound = Some(status);
        self
    }

    pub fn with_right_bound(mut self, status: Status) -> Self {
        self.1.right_bound = Some(status);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for SliceBuilder<T> {}
impl<T> ReqlOpsArray for SliceBuilder<T> {}
impl<T> ReqlOpsDocManipulation for SliceBuilder<T> {}

impl<T> ReqlOps for SliceBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone().with_opts(self.1).into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for SliceBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}
