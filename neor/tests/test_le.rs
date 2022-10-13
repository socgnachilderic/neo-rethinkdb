use neor::{args, r, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_le_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: bool = table
        .get(1)
        .g("view")
        .le(10)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_le_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: bool = r.le(args!([5, 6, 7])).run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained);

    Ok(())
}
