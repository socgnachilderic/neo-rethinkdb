use neor::arguments::{ReconfigureOption, Replicas};
use neor::types::ReconfigureResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_reconfigure_table() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let reconfigure_option = ReconfigureOption::default()
        .shards(2)
        .replicas(Replicas::Int(1));
    let response: ReconfigureResponse = table
        .reconfigure(reconfigure_option)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.reconfigured == 1);

    tear_down(conn, &table_name).await
}
