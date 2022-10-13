use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl WithoutArg) -> Command {
    args.into_without_opts()
        .add_to_cmd(Command::new(TermType::Without))
}

pub trait WithoutArg {
    fn into_without_opts(self) -> CmdOpts;
}

impl<T> WithoutArg for T
where
    T: Serialize,
{
    fn into_without_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl WithoutArg for Command {
    fn into_without_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<T> WithoutArg for Args<T>
where
    T: IntoIterator<Item = Command>,
{
    fn into_without_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct InnerPost {
        id: u8,
        title: String,
    }

    #[tokio::test]
    async fn test_without_data() -> Result<()> {
        let data = Post::get_one_data();
        let data = InnerPost {
            id: data.id,
            title: data.title,
        };
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: InnerPost = table
            .get(1)
            .without(["content", "view"])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }
}
