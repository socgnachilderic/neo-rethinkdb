use neor::types::RebalanceResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_rebalance_table() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: RebalanceResponse = table.rebalance().run(&conn).await?.unwrap().parse()?;

    assert!(response.rebalanced == 1);

    tear_down(conn, &table_name).await
}
