use neor::{r, Converter, Result};

#[tokio::test]
async fn test_bit_xor_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r.expr(6).bit_xor(4).run(&conn).await?.unwrap().parse()?;

    assert!(response == 2);

    Ok(())
}

#[tokio::test]
async fn test_bit_xor_ops_with_command() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r
        .bit_xor(r.expr(6), r.expr(4))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == 2);

    Ok(())
}

#[tokio::test]
async fn test_bit_xor_ops_with_syntax() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = (r.expr(6) ^ r.expr(4)).run(&conn).await?.unwrap().parse()?;

    assert!(response == 2);

    Ok(())
}
