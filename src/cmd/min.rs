use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl MinArg) -> Command {
    let (arg, opts) = args.into_min_opts();
    let mut command = Command::new(TermType::Min);

    if let Some(arg) = arg {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MinArg {
    fn into_min_opts(self) -> (Option<Command>, MinOption);
}

impl MinArg for () {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (None, Default::default())
    }
}

impl<T> MinArg for Args<T>
where
    T: Into<String>,
{
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        let arg = Command::from_json(self.0.into());

        (Some(arg), Default::default())
    }
}

impl MinArg for Func {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (Some(self.0), Default::default())
    }
}

impl MinArg for MinOption {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (None, self)
    }
}

impl MinArg for Command {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (Some(self), Default::default())
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
    use crate::spec::{set_up, tear_down, Post};
    use crate::{args, Result};

    #[tokio::test]
    async fn test_min_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Post = table
            .min(args!("view"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(Some(&data_obtained) == data.last());

        tear_down(conn, &table_name).await
    }
}
