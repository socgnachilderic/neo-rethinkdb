use neor::{r, Converter, Result};

#[tokio::test]
async fn test_month_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let month = r.now().month();
    let month1 = month.value();
    let month2: u8 = month.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(month1 == month2);

    Ok(())
}
