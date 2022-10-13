use neor::{Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down, Post};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct InnerPost {
    id: u8,
    title: String,
}

#[tokio::test]
async fn test_with_fields() -> Result<()> {
    let data: Vec<InnerPost> = Post::get_many_data()
        .into_iter()
        .map(|post| InnerPost {
            id: post.id,
            title: post.title,
        })
        .collect();
    let (conn, table, table_name) = set_up(true).await?;
    let mut data_obtained: Vec<InnerPost> = table
        .with_fields(["id", "title"])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    data_obtained.sort_by(|a, b| a.id.cmp(&b.id));

    assert!(data_obtained == data);

    tear_down(conn, &table_name).await
}
