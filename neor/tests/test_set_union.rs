use neor::{r, Converter, Result};

#[tokio::test]
async fn test_set_union_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [u8; 7] = r
        .expr([10, 20, 30, 40, 50])
        .set_union([60, 70])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == [10, 20, 30, 40, 50, 60, 70]);

    Ok(())
}
