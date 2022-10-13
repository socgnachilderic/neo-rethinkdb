use neor::types::ConfigResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_get_config_info() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    let response: ConfigResponse = table.config().run(&conn).await?.unwrap().parse()?;

    assert!(response.name == table_name);

    tear_down(conn, table_name.as_str()).await
}
