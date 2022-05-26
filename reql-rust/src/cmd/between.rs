use std::borrow::Cow;

use super::{args::Args, StaticString};
use crate::{cmd, Command, Result, types::Status};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct BetweenBuilder<T>(
    pub(crate) Command,
    pub(crate) BetweenOption,
    pub(crate) Option<T>
);

#[derive(Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct BetweenOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

impl<T: Unpin + Serialize + DeserializeOwned> BetweenBuilder<T> {
    pub(crate) fn new(lower_key: impl Serialize, upper_key: impl Serialize) -> Self {
        let arg_lower_key = Command::from_json(lower_key);
        let arg_upper_key = Command::from_json(upper_key);
        let command = Command::new(TermType::Between)
            .with_arg(arg_lower_key)
            .with_arg(arg_upper_key);

        Self(command, BetweenOption::default(), None)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> Result<Option<serde_json::Value>> {
        self.make_query(arg)
            .try_next()
            .await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = Result<serde_json::Value>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, serde_json::Value>(arg)
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.1.index = Some(index.static_string());
        self
    }

    pub fn with_left_bound(mut self, status: Status) -> Self {
        self.1.left_bound = Some(status);
        self
    }

    pub fn with_right_bound(mut self, status: Status) -> Self {
        self.1.right_bound = Some(status);
        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<BetweenOption>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<BetweenOption> {
        Self::new(TermType::Between).with_arg(self).into_arg()
    }
}

impl Arg for Args<(Command, BetweenOption)> {
    fn arg(self) -> cmd::Arg<BetweenOption> {
        let Args((query, opts)) = self;
        query.arg().with_opts(opts)
    }
}

impl<T> Arg for Args<(T, T)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<BetweenOption> {
        let Args((min, max)) = self;
        let max = Command::from_json(max);
        Command::from_json(min).arg().with_arg(max)
    }
}

impl<T> Arg for Args<(T, T, BetweenOption)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<BetweenOption> {
        let Args((min, max, opts)) = self;
        let max = Command::from_json(max);
        Command::from_json(min).arg().with_arg(max).with_opts(opts)
    }
}
