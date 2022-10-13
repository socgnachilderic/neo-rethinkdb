use neor::{args, Converter, Result};

use common::{Comment, CommentWithPost};

mod common;

#[tokio::test]
pub async fn test_zip_ops() -> Result<()> {
    let data = CommentWithPost {
        id: 1,
        text: "comment4".to_string(),
        post_id: 1,
        title: "title1".to_string(),
        content: Some("content1".to_string()),
        view: 10,
    };
    let (conn, comment_table, post_table, comment_tablename, post_tablename) =
        Comment::own_set_up().await?;

    let response: Vec<CommentWithPost> = comment_table
        .eq_join(args!("post_id", post_table))
        .zip()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.len() > 0);
    assert_eq!(response.first(), Some(&data));

    Comment::own_tear_down(conn, comment_tablename, post_tablename).await
}
