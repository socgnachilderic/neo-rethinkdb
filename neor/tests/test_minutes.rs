use neor::{r, Converter, Result};

#[tokio::test]
async fn test_minutes_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let minutes = r.now().minutes();
    let minutes1 = minutes.value();
    let minutes2: u8 = minutes.cmd().run(&conn).await?.unwrap().parse()?;

    assert!(minutes1 == minutes2);

    Ok(())
}
