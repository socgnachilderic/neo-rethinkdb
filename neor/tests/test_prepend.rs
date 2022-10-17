use neor::{r, Converter, Result};

#[tokio::test]
async fn test_prepend_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 6] = r
        .expr([10, 20, 30, 40, 50])
        .prepend(0)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [0, 10, 20, 30, 40, 50]);

    Ok(())
}
