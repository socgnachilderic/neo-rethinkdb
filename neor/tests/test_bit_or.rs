use neor::{r, Converter, Result};

#[tokio::test]
async fn test_bit_or_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r.expr(5).bit_or(3).run(&conn).await?.unwrap().parse()?;

    assert!(response == 7);

    Ok(())
}

#[tokio::test]
async fn test_bit_or_ops_with_command() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r
        .bit_or(r.expr(5), r.expr(3))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == 7);

    Ok(())
}

#[tokio::test]
async fn test_bit_or_ops_with_syntax() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = (r.expr(5) | r.expr(3)).run(&conn).await?.unwrap().parse()?;

    assert!(response == 7);

    Ok(())
}
