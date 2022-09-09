use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::{types::AnyParam, Command, Func};

pub(crate) fn new(args: impl FilterArg) -> Command {
    let (arg, opts) = args.into_filter_opts();

    Command::new(TermType::Filter).with_arg(arg).with_opts(opts)
}

pub trait FilterArg {
    fn into_filter_opts(self) -> (Command, FilterOption);
}

impl FilterArg for AnyParam {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self.into(), Default::default())
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

impl FilterArg for (AnyParam, FilterOption) {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self.0.into(), self.1)
    }
}

impl FilterArg for (Func, FilterOption) {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        let Func(func) = self.0;

        (func, self.1)
    }
}

impl FilterArg for (Command, FilterOption) {
    fn into_filter_opts(self) -> (Command, FilterOption) {
        (self.0, self.1)
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
#[non_exhaustive]
pub struct FilterOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::types::AnyParam;
    use crate::Result;

    #[tokio::test]
    async fn test_filter_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_filtered: Vec<Post> = table
            .clone()
            .filter(AnyParam::new(json!({"view": 2})))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_filtered.len() == 2);
        assert!(data_filtered.first() == data.get(3));
        assert!(data_filtered.last() == data.get(1));

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
