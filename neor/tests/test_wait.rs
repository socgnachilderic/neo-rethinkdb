use neor::types::WaitResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_wait_table() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: WaitResponse = table.wait(()).run(&conn).await?.unwrap().parse()?;

    assert!(response.ready == 1);

    tear_down(conn, &table_name).await
}
