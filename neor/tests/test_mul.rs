use neor::{r, Converter, Result};

#[tokio::test]
async fn test_mul_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response1: u8 = (r.expr(2) * 2).run(&conn).await?.unwrap().parse()?;
    let response2: Vec<String> = (r.expr(["This", "is", "the", "song", "that", "never", "ends."])
        * 100)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response1, 4);
    assert_eq!(response2.len(), 700);

    Ok(())
}
