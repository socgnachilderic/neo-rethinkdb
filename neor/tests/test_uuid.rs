use neor::{r, Converter, Result};

#[tokio::test]
async fn test_uuid_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: String = r.uuid(()).run(&conn).await?.unwrap().parse()?;

    assert!(!response.is_empty());

    Ok(())
}
