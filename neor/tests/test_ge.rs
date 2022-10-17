use neor::{args, r, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_ge_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: bool = table
        .get(1)
        .g("view")
        .ge(10)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_ge_data_r() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: bool = r.ge(args!([7, 6, 5])).run(&conn).await?.unwrap().parse()?;

    assert!(response);

    Ok(())
}
