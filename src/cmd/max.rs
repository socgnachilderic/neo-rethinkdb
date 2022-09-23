use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl MaxArg) -> Command {
    let (arg1, arg2, opts) = args.into_max_opts();
    let mut command = Command::new(TermType::Max);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MaxArg {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption);
}

impl MaxArg for &str {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        let arg = Command::from_json(self);

        (None, Some(arg), Default::default())
    }
}

impl MaxArg for Func {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        (None, Some(self.0), Default::default())
    }
}

impl MaxArg for MaxOption {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        (None, None, self)
    }
}

impl MaxArg for (Command, &str) {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        let arg = Command::from_json(self.1);

        (Some(self.0), Some(arg), Default::default())
    }
}

impl MaxArg for (Command, Func) {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        (Some(self.0), None, Default::default())
    }
}

impl MaxArg for (Command, MaxOption) {
    fn into_max_opts(self) -> (Option<Command>, Option<Command>, MaxOption) {
        (Some(self.0), None, self.1)
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
    use crate::Result;

    #[tokio::test]
    async fn test_max_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Post = table.max("view").run(&conn).await?.unwrap().parse()?;

        assert!(Some(&data_obtained) == data.first());

        tear_down(conn, &table_name).await
    }
}
