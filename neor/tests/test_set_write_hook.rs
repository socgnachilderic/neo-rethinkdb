use neor::types::SetWriteHookResponse;
use neor::{func, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_set_write_hook_ops() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;

    let response: SetWriteHookResponse = table
        .set_write_hook(func!(|_, _, new_val| new_val))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response.created, Some(1));

    tear_down(conn, &table_name).await
}
