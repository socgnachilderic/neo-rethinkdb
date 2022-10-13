use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_skip_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Vec<Post> = table
        .skip(data.len() - 1)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained.first() == data.first());

    tear_down(conn, &table_name).await
}
