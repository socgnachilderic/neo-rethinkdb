use neor::{r, Converter, Result};

#[tokio::test]
async fn test_set_difference_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 3] = r
        .expr([10, 20, 30, 40, 50])
        .set_difference([20, 40])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [10, 30, 50]);

    Ok(())
}
