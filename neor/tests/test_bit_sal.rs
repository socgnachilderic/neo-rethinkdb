use neor::{r, Converter, Result};

#[tokio::test]
async fn test_bit_sal_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r.expr(5).bit_sal(4).run(&conn).await?.unwrap().parse()?;

    assert!(response == 80);

    Ok(())
}

#[tokio::test]
async fn test_bit_sal_ops_with_command() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: u8 = r
        .bit_sal(r.expr(5), r.expr(4))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == 80);

    Ok(())
}
