use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl FilterArg) -> Command {
    let (arg, opts) = args.into_filter_opts();

    Command::new(TermType::Filter).with_arg(arg).with_opts(opts)
}

pub trait FilterArg {
    fn into_filter_opts(self) -> (Command, FilterOption);
}

impl<T> FilterArg for T
where
    T: Serialize,
{
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (Command::from_json(self), Default::default())
    }
}

impl FilterArg for Func {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self.0, Default::default())
    }
}

impl FilterArg for Command {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self, Default::default())
    }
}

impl<T> FilterArg for Args<(T, FilterOption)>
where
    T: Serialize,
{
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (Command::from_json(self.0 .0), self.0 .1)
    }
}

impl FilterArg for Args<(Func, FilterOption)> {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        let Func(func) = self.0 .0;

        (func, self.0 .1)
    }
}

impl FilterArg for Args<(Command, FilterOption)> {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self.0 .0, self.0 .1)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FilterOption {
    /// - If `default` is set to `true`, documents with missing
    /// fields will be returned rather than skipped.
    /// - If `default` is set to `r.error()`, an `ReqlRuntimeError` will
    /// be thrown when a document with a missing field is tested.
    /// - If `default` is set to `false` (the default),
    /// documents with missing fields will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl FilterOption {
    pub fn default_(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_filter_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_filtered: Vec<Post> = table
            .clone()
            .filter(json!({"view": 2}))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_filtered.len() == 2);
        assert!(data_filtered.first() == data.get(3));
        assert!(data_filtered.last() == data.get(1));

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_filter_data_with_func() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_filtered: Vec<Post> = table
            .clone()
            .filter(func!(|user| user.g("view").eq(r.expr(2))))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_filtered.len() == 2);
        assert!(data_filtered.first() == data.get(3));
        assert!(data_filtered.last() == data.get(1));

        tear_down(conn, &table_name).await
    }
}
