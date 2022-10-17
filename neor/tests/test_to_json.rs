use neor::{Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_to_json_string() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: String = table.get(1).to_json().run(&conn).await?.unwrap().parse()?;

    assert!(!response.is_empty());

    tear_down(conn, &table_name).await
}
