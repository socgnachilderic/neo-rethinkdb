use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_is_empty() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: bool = table.is_empty().run(&conn).await?.unwrap().parse()?;

    assert!(!data_obtained);

    tear_down(conn, &table_name).await
}
