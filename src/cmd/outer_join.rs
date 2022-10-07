use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(other_table: Command, func: Func) -> Command {
    let Func(func) = func;

    Command::new(TermType::OuterJoin)
        .with_arg(other_table)
        .with_arg(func)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{Comment, Post};
    use crate::types::JoinResponse;
    use crate::Result;

    #[tokio::test]
    pub async fn test_eq_join_ops() -> Result<()> {
        let data = JoinResponse {
            left: Some(Comment {
                id: 4,
                text: "comment4".to_string(),
                post_id: 2,
            }),
            right: Some(Post {
                id: 2,
                title: "title2".to_string(),
                content: Some("content2".to_string()),
                view: 2,
            }),
        };
        let (conn, comment_table, post_table, comment_tablename, post_tablename) =
            Comment::own_set_up().await?;

        let response: Vec<JoinResponse<Comment, Post>> = comment_table
            .outer_join(
                post_table,
                func!(|comment, post| comment.g("post_id").eq(post.g("id"))),
            )
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.len() > 0);
        assert_eq!(response.first(), Some(&data));

        Comment::own_tear_down(conn, comment_tablename, post_tablename).await
    }
}
