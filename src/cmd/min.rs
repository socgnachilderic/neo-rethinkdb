use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl MinArg) -> Command {
    let (arg1, arg2, opts) = args.into_min_opts();
    let mut command = Command::new(TermType::Min);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MinArg {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption);
}

impl MinArg for &str {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        let arg = Command::from_json(self);

        (None, Some(arg), Default::default())
    }
}

impl MinArg for Func {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        (None, Some(self.0), Default::default())
    }
}

impl MinArg for MinOption {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        (None, None, self)
    }
}

impl MinArg for (Command, &str) {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        let arg = Command::from_json(self.1);

        (Some(self.0), Some(arg), Default::default())
    }
}

impl MinArg for (Command, Func) {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        (Some(self.0), None, Default::default())
    }
}

impl MinArg for (Command, MinOption) {
    fn into_min_opts(self) -> (Option<Command>, Option<Command>, MinOption) {
        (Some(self.0), None, self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct MinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_min_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Post = table.min("view").run(&conn).await?.unwrap().parse()?;

        assert!(Some(&data_obtained) == data.last());

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
