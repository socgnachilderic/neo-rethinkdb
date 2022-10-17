use neor::types::GroupedStream;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_group_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: GroupedStream<String, Post> =
        table.group("title").run(&conn).await?.unwrap().parse()?;

    let response = response.collect();

    assert!(response.len() == 4);

    tear_down(conn, &table_name).await
}
