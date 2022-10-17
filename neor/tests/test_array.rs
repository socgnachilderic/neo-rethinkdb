use neor::{r, Converter, Result};

#[tokio::test]
async fn test_array_data() -> Result<()> {
    let data = vec![1u8, 2, 3, 4];
    let conn = r.connection().connect().await?;
    let response: Vec<u8> = r
        .array(data.iter().map(|value| r.expr(value)))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    Ok(())
}
