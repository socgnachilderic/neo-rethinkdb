use common::{set_up, tear_down};
use neor::{Converter, Result};

mod common;

#[tokio::test]
async fn test_list_index() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    table.index_create("author").run(&conn).await?;
    let index_list: Vec<String> = table.index_list().run(&conn).await?.unwrap().parse()?;

    assert!(index_list.len() > 0);

    tear_down(conn, &table_name).await
}
