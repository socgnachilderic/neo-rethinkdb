use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_sum_data() -> Result<()> {
    let data: u8 = Post::get_many_data().iter().map(|post| post.view).sum();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: u8 = table.sum("view").run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained == data);

    tear_down(conn, &table_name).await
}
