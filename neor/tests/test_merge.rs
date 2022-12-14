use std::collections::HashMap;

use neor::{args, func, r, Command, Converter, Result, Session};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common::*;

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Comment {
    id: u8,
    post_id: u8,
    message: String,
}

impl Comment {
    fn new(id: u8, post_id: u8, message: &str) -> Self {
        Self {
            id,
            post_id,
            message: String::from(message),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct MergePostToComment {
    id: u8,
    post_id: u8,
    message: String,
    post: Post,
}

#[tokio::test]
async fn test_merge_ops() -> Result<()> {
    let (
        conn,
        comment_table,
        post_table,
        merged_post_comments,
        comment_table_name,
        post_table_name,
    ) = set_up2().await?;

    let response: Vec<MergePostToComment> = comment_table
        .merge(func!(|comment| {
            let mut posts = HashMap::new();

            posts.insert("post", post_table.get(comment.g("post_id")));

            r.hash_map(posts)
        }))
        .order_by("id")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == merged_post_comments);

    tear_down2(conn, comment_table_name, post_table_name).await
}

#[tokio::test]
async fn test_merge_ops_multi() -> Result<()> {
    let (conn, comment_table, post_table, _, comment_table_name, post_table_name) =
        set_up2().await?;

    let response = post_table
        .get(1)
        .merge(args!([comment_table.get(1), comment_table.get(2),]))
        .run(&conn)
        .await?;

    assert!(response.is_some());

    tear_down2(conn, comment_table_name, post_table_name).await
}

async fn set_up2() -> Result<(
    Session,
    Command,
    Command,
    Vec<MergePostToComment>,
    String,
    String,
)> {
    let comment_table_name = Uuid::new_v4().to_string();
    let posts = Post::get_many_data();
    let comments = vec![
        Comment::new(1, 1, "Hello"),
        Comment::new(2, 1, "Hello"),
        Comment::new(3, 2, "Bueno dias"),
    ];
    let merged_post_comments: Vec<MergePostToComment> = comments
        .iter()
        .map(|comment| MergePostToComment {
            id: comment.id,
            post_id: comment.post_id,
            message: String::from(&comment.message),
            post: posts
                .clone()
                .into_iter()
                .find(|post| post.id == comment.post_id)
                .unwrap(),
        })
        .collect();

    let (conn, post_table, post_table_name) = set_up(true).await?;
    r.table_create(comment_table_name.as_str())
        .run(&conn)
        .await?;
    let comment_table = r.table(comment_table_name.as_str());
    comment_table.insert(comments).run(&conn).await?;

    Ok((
        conn,
        comment_table,
        post_table,
        merged_post_comments,
        comment_table_name,
        post_table_name,
    ))
}

async fn tear_down2(
    conn: Session,
    comment_table_name: String,
    post_table_name: String,
) -> Result<()> {
    r.table_drop(&comment_table_name).run(&conn).await?;
    tear_down(conn, &post_table_name).await
}
