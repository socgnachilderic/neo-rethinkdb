use super::StaticString;
use crate::{Command, Result};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct GetAllBuilder<T>(
    pub(crate) Command,
    pub(crate) GetAllOption,
    pub(crate) Option<T>,
);

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct GetAllOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + Serialize + DeserializeOwned> GetAllBuilder<T> {
    pub(crate) fn new(index_keys: &[&str]) -> Self {
        let mut command = Command::new(TermType::GetAll);

        for index_key in index_keys {
            let args = Command::from_json(*index_key);
            command = command.with_arg(args);
        }

        Self(command, GetAllOption::default(), None)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<Option<T>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = Result<Option<T>>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Option<T>>(arg)
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
