use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_max_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Post = table.max("view").run(&conn).await?.unwrap().parse()?;

    assert!(Some(&data_obtained) == data.first());

    tear_down(conn, &table_name).await
}
