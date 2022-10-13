use neor::types::SyncResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_sync_ops() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let sync_response: SyncResponse = table.sync().run(&conn).await?.unwrap().parse()?;

    assert!(sync_response.synced == 1);

    tear_down(conn, &table_name).await
}
