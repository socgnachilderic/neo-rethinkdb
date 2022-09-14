use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl GetAllArg) -> Command {
    let (args, opts) = args.into_get_all_opts();

    args.add_to_cmd(Command::new(TermType::GetAll))
        .with_opts(opts)
}

pub trait GetAllArg {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption);
}

impl<T: Serialize> GetAllArg for Vec<T> {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        let keys = self
            .into_iter()
            .map(|key| Command::from_json(key))
            .collect();

        (CmdOpts::Many(keys), Default::default())
    }
}

impl<T: Serialize> GetAllArg for (Vec<T>, GetAllOption) {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        let keys = self
            .0
            .into_iter()
            .map(|key| Command::from_json(key))
            .collect();

        (CmdOpts::Many(keys), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GetAllOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    use super::GetAllOption;

    #[tokio::test]
    async fn test_get_all() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up( true).await?;

        table.clone().sync().run(&conn).await?;

        let data_get: Vec<Post> = table
            .get_all((vec!["title4"], GetAllOption::default().index("title")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_get.len() == 2);
        assert!(data_get.first() == data.get(3));
        assert!(data_get.last() == data.last());

        tear_down(conn, &table_name).await
    }
}
