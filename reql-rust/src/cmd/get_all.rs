use std::borrow::Cow;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl GetAllArg) -> Command {
    let (args, opts) = args.into_get_all_opts();
    let mut command = Command::new(TermType::GetAll);

    for arg in args {
        command = command.with_arg(arg);
    }

    command.with_opts(opts)
}

pub trait GetAllArg {
    fn into_get_all_opts(self) -> (Vec<Command>, GetAllOption);
}

impl<T: Serialize> GetAllArg for Vec<T> {
    fn into_get_all_opts(self) -> (Vec<Command>, GetAllOption) {
        let keys = self
            .into_iter()
            .map(|key| Command::from_json(key))
            .collect();

        (keys, Default::default())
    }
}

impl<T: Serialize> GetAllArg for (Vec<T>, GetAllOption) {
    fn into_get_all_opts(self) -> (Vec<Command>, GetAllOption) {
        let keys = self
            .0
            .into_iter()
            .map(|key| Command::from_json(key))
            .collect();

        (keys, self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct GetAllOption {
    pub index: Option<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    use super::GetAllOption;

    #[tokio::test]
    async fn test_get_all() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(TABLE_NAMES[1], true).await?;

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

        tear_down(conn, TABLE_NAMES[1]).await
    }
}
