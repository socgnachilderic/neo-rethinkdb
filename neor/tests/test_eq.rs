use neor::{args, r, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_eq_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: bool = table
        .get(1)
        .g("title")
        .eq("title1")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_eq_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: bool = r.eq(args!([5, 5, 5])).run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained);

    Ok(())
}
