use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_min_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: Post = table.min("view").run(&conn).await?.unwrap().parse()?;

    assert!(Some(&response) == data.last());

    tear_down(conn, &table_name).await
}
