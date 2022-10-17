use neor::{r, Converter, Result};

#[tokio::test]
async fn test_set_intersection_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 2] = r
        .expr([10, 20, 30, 40, 50])
        .set_intersection([20, 40])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [20, 40]);

    Ok(())
}
