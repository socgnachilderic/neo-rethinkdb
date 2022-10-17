use std::collections::HashMap;

use neor::{func, r, Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down, Post};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct InnerPost {
    title: String,
    content: String,
}

#[tokio::test]
async fn test_default_ops() -> Result<()> {
    let data: Vec<InnerPost> = Post::get_many_data()
        .into_iter()
        .map(|post| InnerPost {
            title: post.title,
            content: if let Some(content) = post.content {
                content
            } else {
                "Anonymous".to_owned()
            },
        })
        .collect();
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<InnerPost> = table
        .order_by(r.expr("title"))
        .map(func!(|doc| {
            let mut post = HashMap::new();
            post.insert("title", doc.g("title"));
            post.insert("content", doc.g("content").default("Anonymous"));
            r.hash_map(post)
        }))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    tear_down(conn, &table_name).await
}
