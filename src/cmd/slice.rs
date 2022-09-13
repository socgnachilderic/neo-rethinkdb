use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::Status;
use crate::Command;

pub(crate) fn new(args: impl SliceArg) -> Command {
    let (start_offset, end_offset, opts) = args.into_slice_opts();
    let mut command = Command::new(TermType::Slice).with_arg(start_offset);

    if let Some(end_offset) = end_offset {
        command = command.with_arg(end_offset);
    }

    command.with_opts(opts)
}

pub trait SliceArg {
    fn into_slice_opts(self) -> (Command, Option<Command>, SliceOption);
}

impl SliceArg for isize {
    fn into_slice_opts(self) -> (Command, Option<Command>, SliceOption) {
        (Command::from_json(self), None, Default::default())
    }
}

impl SliceArg for (isize, isize) {
    fn into_slice_opts(self) -> (Command, Option<Command>, SliceOption) {
        (
            Command::from_json(self.0),
            Some(Command::from_json(self.1)),
            Default::default(),
        )
    }
}

impl SliceArg for (isize, SliceOption) {
    fn into_slice_opts(self) -> (Command, Option<Command>, SliceOption) {
        (Command::from_json(self), None, self.1)
    }
}

impl SliceArg for (isize, isize, SliceOption) {
    fn into_slice_opts(self) -> (Command, Option<Command>, SliceOption) {
        (
            Command::from_json(self.0),
            Some(Command::from_json(self.1)),
            self.2,
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct SliceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[cfg(test)]
mod tests {
    use crate::cmd::order_by::OrderByOption;
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_slice_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: Vec<Post> = table
            .order_by(OrderByOption::default().index("id"))
            .slice((4, 5))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.last() == data.last());

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
