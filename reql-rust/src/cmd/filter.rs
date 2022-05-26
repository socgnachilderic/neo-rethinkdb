use crate::{Command, Func, Result};
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct FilterBuilder<T>(Command, FilterOption, Option<T>);

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct FilterOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl<T: Unpin + Serialize + DeserializeOwned> FilterBuilder<T> {
    pub fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Filter).with_arg(func);

        Self(command, FilterOption::default(), None)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<T>> {
        self.0.with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, T>(arg)
            .try_next()
            .await
    }

    pub fn with_default(mut self, default: bool) -> Self {
        self.1.default = Some(default);
        self
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T> Into<Command> for FilterBuilder<T> {
    fn into(self) -> Command {
        self.0
    }
}
