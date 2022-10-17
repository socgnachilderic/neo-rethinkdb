use neor::{r, Converter, Result};

#[tokio::test]
async fn test_bit_not_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: i32 = r.expr(7).bit_not().run(&conn).await?.unwrap().parse()?;

    assert!(response == -8);

    Ok(())
}

#[tokio::test]
async fn test_bit_not_ops_with_command() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: i32 = r.bit_not(r.expr(7)).run(&conn).await?.unwrap().parse()?;

    assert!(response == -8);

    Ok(())
}
