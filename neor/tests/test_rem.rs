use neor::{r, Converter, Result};

#[tokio::test]
async fn test_rem_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = (r.expr(2) % 2).run(&conn).await?.unwrap().parse()?;

    assert_eq!(response, 0);

    Ok(())
}
