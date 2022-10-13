use neor::types::GetWriteHookResponse;
use neor::{func, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_get_write_hook_ops() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    table
        .clone()
        .set_write_hook(func!(|_, _, new_val| new_val))
        .run(&conn)
        .await?;

    let response: GetWriteHookResponse =
        table.get_write_hook().run(&conn).await?.unwrap().parse()?;

    assert!(!response.query.is_empty());

    tear_down(conn, &table_name).await
}
