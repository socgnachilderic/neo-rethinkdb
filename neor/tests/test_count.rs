use common::{set_up, tear_down, Post};
use neor::{Converter, Result};

mod common;

#[tokio::test]
async fn test_count_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: usize = table.count(()).run(&conn).await?.unwrap().parse()?;

    assert!(response == data.len());

    tear_down(conn, &table_name).await
}
