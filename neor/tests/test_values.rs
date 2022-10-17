use neor::Result;

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_values_fields() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response = table.get(1).values().run(&conn).await?.unwrap();

    assert!(response.is_array());

    tear_down(conn, &table_name).await
}
