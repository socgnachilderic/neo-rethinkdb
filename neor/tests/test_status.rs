use neor::types::StatusResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_status_table() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: StatusResponse = table.status().run(&conn).await?.unwrap().parse()?;

    assert!(response.name.unwrap() == table_name);

    tear_down(conn, &table_name).await
}
