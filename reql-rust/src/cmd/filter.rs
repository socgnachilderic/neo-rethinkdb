use crate::{Command, Func, Result};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct FilterBuilder<T>(
    pub(crate) Command,
    pub(crate) FilterOption,
    pub(crate) Option<T>,
);

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub(crate) struct FilterOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl<T: Unpin + Serialize + DeserializeOwned> FilterBuilder<T> {
    pub(crate) fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Filter).with_arg(func);

        Self(command, FilterOption::default(), None)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<Vec<T>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = Result<Vec<T>>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<T>>(arg)
    }

    pub fn with_default(mut self, default: bool) -> Self {
        self.1.default = Some(default);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T> Into<Command> for FilterBuilder<T> {
    fn into(self) -> Command {
        self.0
    }
}
