use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::{Command, Func};

#[derive(Debug, Clone)]
pub struct CountBuilder(pub(crate) Command);

impl CountBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Count);
        Self(command)
    }

    pub(crate) fn new_by_value(value: impl Serialize) -> Self {
        let arg = Command::from_json(value);
        let command = Command::new(TermType::Count).with_arg(arg);
        Self(command)
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Count).with_arg(func);
        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<usize>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<usize>> {
        self.0.into_arg::<()>().into_cmd().run::<_, usize>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
