use neor::{func, r, Converter, Result};

#[tokio::test]
async fn test_map_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: Vec<u8> = r
        .expr([1, 2, 3, 4, 5])
        .map(func!(|val| val.clone() * val))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == vec![1, 4, 9, 16, 25]);

    Ok(())
}
