use neor::{r, Converter, Result};

#[tokio::test]
async fn test_append_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 6] = r
        .expr([10, 20, 30, 40, 50])
        .append(100)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [10, 20, 30, 40, 50, 100]);

    Ok(())
}
