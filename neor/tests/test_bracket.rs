use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_bracket_data() -> Result<()> {
    let data = Post::get_one_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: String = table
        .get(1)
        .bracket("title")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained == data.title);

    tear_down(conn, &table_name).await
}
