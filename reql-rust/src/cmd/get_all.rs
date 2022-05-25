use super::StaticString;
use crate::{Command, Result};
use futures::TryStreamExt;
use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::{Serialize, de::DeserializeOwned};
use std::borrow::Cow;

pub struct GetAllBuilder<T>(Command, GetAllOption, Option<T>);

#[derive(Debug, Clone, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct GetAllOption {
    pub index: Option<Cow<'static, str>>,
}

impl<T: Unpin + Serialize + DeserializeOwned> GetAllBuilder<T> {
    pub fn new(index_keys: &[&str]) -> Self {
        let mut command = Command::new(TermType::GetAll);
        
        for index_key in index_keys {
            let args = Command::from_json(*index_key);
            command = command.with_arg(args);
        }

        Self(command, GetAllOption::default(), None)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<Option<T>>> {
        let command = self.0.with_opts(self.1)
            .into_arg::<()>()
            .into_cmd();
        
        command.run::<_, Option<T>>(arg).try_next().await
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.1.index = Some(index.static_string());
        self
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
