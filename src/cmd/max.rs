use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl MaxArg) -> Command {
    let (arg, opts) = args.into_max_opts();
    let mut command = Command::new(TermType::Max);

    if let Some(arg) = arg {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MaxArg {
    fn into_max_opts(self) -> (Option<Command>, MaxOption);
}

impl MaxArg for () {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (None, Default::default())
    }
}

impl<T> MaxArg for Args<T> where T: Into<String> {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        let arg = Command::from_json(self.0.into());

        (Some(arg), Default::default())
    }
}

impl MaxArg for Func {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (Some(self.0), Default::default())
    }
}

impl MaxArg for MaxOption {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (None, self)
    }
}

impl MaxArg for Command {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (Some(self), Default::default())
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct MaxOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::{Result, args};

    #[tokio::test]
    async fn test_max_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Post = table.max(args!("view")).run(&conn).await?.unwrap().parse()?;

        assert!(Some(&data_obtained) == data.first());

        tear_down(conn, &table_name).await
    }
}
