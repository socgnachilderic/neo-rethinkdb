use crate::{types::IndexResponseType, Command, Func};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct IndexCreateBuilder(pub(crate) Command, pub(crate) IndexCreateOption);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
pub(crate) struct IndexCreateOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
}

impl IndexCreateBuilder {
    pub(crate) fn new(index_name: &str) -> Self {
        let args = Command::from_json(index_name);
        let command = Command::new(TermType::IndexCreate).with_arg(args);

        Self(command, IndexCreateOption::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<IndexResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<IndexResponseType>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, IndexResponseType>(arg)
    }

    pub fn with_func(mut self, func: Func) -> Self {
        let Func(func) = func;
        self.0 = self.0.with_arg(func);
        self
    }

    pub fn with_query(mut self, query: Command) -> Self {
        let Func(func) = Func::row(query);
        self.0 = self.0.with_arg(func);
        self
    }

    pub fn with_multi(mut self, multi: bool) -> Self {
        self.1.multi = Some(multi);
        self
    }

    pub fn with_geo(mut self, geo: bool) -> Self {
        self.1.geo = Some(geo);
        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}
