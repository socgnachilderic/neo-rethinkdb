use neor::types::JoinResponse;
use neor::{func, Converter, Result};

use common::{Comment, Post};

mod common;

#[tokio::test]
pub async fn test_outer_join_ops() -> Result<()> {
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
