use neor::types::MutationResponse;
use neor::{Converter, Result};
use serde_json::json;

use common::*;

mod common;

#[tokio::test]
async fn test_update_docs() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table
        .get(1)
        .update(json!({"view": 0}))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.replaced == 1);

    tear_down(conn, &table_name).await
}
