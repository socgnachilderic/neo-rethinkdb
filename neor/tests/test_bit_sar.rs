use neor::{r, Converter, Result};

#[tokio::test]
async fn test_bit_sar_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r.expr(32).bit_sar(3).run(&conn).await?.unwrap().parse()?;

    assert!(response == 4);

    Ok(())
}

#[tokio::test]
async fn test_bit_sar_ops_with_command() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r
        .bit_sar(r.expr(32), r.expr(3))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == 4);

    Ok(())
}
