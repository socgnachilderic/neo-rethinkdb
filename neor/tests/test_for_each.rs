use neor::types::MutationResponse;
use neor::{func, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_for_each_opts() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table
        .clone()
        .for_each(func!(|doc| table.get(doc.g("id")).delete(())))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.deleted == 5);

    tear_down(conn, &table_name).await
}
