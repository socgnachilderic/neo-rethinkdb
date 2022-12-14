use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_keys_values() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<String> = table.get(1).keys().run(&conn).await?.unwrap().parse()?;

    assert!(response == vec!["content", "id", "title", "view"]);

    tear_down(conn, &table_name).await
}
