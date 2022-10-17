use neor::{r, Converter, Result};

#[tokio::test]
async fn test_timezone_ops() -> Result<()> {
    let conn = r.connection().connect().await?;

    let timezone = r.now().timezone();
    let timezone2: String = timezone.cmd().run(&conn).await?.unwrap().parse()?;

    assert_ne!(timezone.value().to_string(), timezone2);

    Ok(())
}
