use neor::{Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down, Post};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct InnerPost {
    id: u8,
    title: String,
}

#[tokio::test]
async fn test_pluck_data() -> Result<()> {
    let data = Post::get_one_data();
    let data = InnerPost {
        id: data.id,
        title: data.title,
    };
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: InnerPost = table
        .get(1)
        .pluck(["id", "title"])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained == data);

    tear_down(conn, &table_name).await
}
