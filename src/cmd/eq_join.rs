use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl EqJoinArg) -> Command {
    let (arg, right_table, opts) = args.into_eq_join_opts();

    Command::new(TermType::EqJoin)
        .with_arg(arg)
        .with_arg(right_table)
        .with_opts(opts)
}

pub trait EqJoinArg {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption);
}

impl<T> EqJoinArg for Args<(T, Command)>
where
    T: Into<String>,
{
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (
            Command::from_json(self.0 .0.into()),
            self.0 .1,
            Default::default(),
        )
    }
}

impl EqJoinArg for Args<(Func, Command)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0 .0;

        (func, self.0 .1, Default::default())
    }
}

impl EqJoinArg for Args<(Command, Command)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0 .0, self.0 .1, Default::default())
    }
}

impl<T> EqJoinArg for Args<(T, Command, EqJoinOption)>
where
    T: Into<String>,
{
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (Command::from_json(self.0 .0.into()), self.0 .1, self.0 .2)
    }
}

impl EqJoinArg for Args<(Func, Command, EqJoinOption)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0 .0;

        (func, self.0 .1, self.0 .2)
    }
}

impl EqJoinArg for Args<(Command, Command, EqJoinOption)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

#[derive(
    Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
#[non_exhaustive]
pub struct EqJoinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{Comment, Post};
    use crate::types::JoinResponse;
    use crate::{args, Result};

    #[tokio::test]
    pub async fn test_eq_join_ops() -> Result<()> {
        let data = JoinResponse {
            left: Some(Comment {
                id: 5,
                text: "comment4".to_string(),
                post_id: 1,
            }),
            right: Some(Post {
                id: 1,
                title: "title1".to_string(),
                content: Some("content1".to_string()),
                view: 10,
            }),
        };
        let (conn, comment_table, post_table, comment_tablename, post_tablename) =
            Comment::own_set_up().await?;

        let response: Vec<JoinResponse<Comment, Post>> = comment_table
            .eq_join(args!("post_id", post_table))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.len() > 0);
        assert_eq!(response.first(), Some(&data));

        Comment::own_tear_down(conn, comment_tablename, post_tablename).await
    }
}
