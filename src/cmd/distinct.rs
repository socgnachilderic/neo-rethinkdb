use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl DistinctArg) -> Command {
    let (args, opts) = args.into_distinct_opts();
    let mut command = Command::new(TermType::Distinct);

    if let Some(arg) = args {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait DistinctArg {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption);
}

impl DistinctArg for () {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (None, Default::default())
    }
}

impl DistinctArg for DistinctOption {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (None, self)
    }
}

impl DistinctArg for Command {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (Some(self), Default::default())
    }
}

impl DistinctArg for (Command, DistinctOption) {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (Some(self.0), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DistinctOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    use super::DistinctOption;

    #[tokio::test]
    async fn test_distinct_data() -> Result<()> {
        let mut data = Post::get_many_data()
            .into_iter()
            .map(|post| post.title)
            .collect::<Vec<String>>();
        data.pop();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: Vec<String> = table
            .distinct(DistinctOption::default().index("title"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }
}
