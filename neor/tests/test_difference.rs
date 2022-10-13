use neor::{r, Converter, Result};

#[tokio::test]
async fn test_difference_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 4] = r
        .expr([10, 20, 30, 40, 50])
        .difference([30, 70])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [10, 20, 40, 50]);

    Ok(())
}
