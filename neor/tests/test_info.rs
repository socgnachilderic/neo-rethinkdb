use neor::types::{InfoResponse, TypeOf};
use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_info_table() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    let data_obtained: InfoResponse = table.info().run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained.typ == TypeOf::Table);

    tear_down(conn, &table_name).await
}
