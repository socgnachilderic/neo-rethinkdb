use neor::{args, r, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_eq_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: bool = table
        .get(1)
        .g("title")
        .eq("title1")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_eq_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: bool = r.eq(args!([5, 5, 5])).run(&conn).await?.unwrap().parse()?;

    assert!(response);

    Ok(())
}
