use neor::types::UngroupItem;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_ungroup_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Vec<UngroupItem<String, Post>> = table
        .group("title")
        .ungroup()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained.len() == 4);

    tear_down(conn, &table_name).await
}
