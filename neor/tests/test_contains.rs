use neor::{r, Converter, Result};

#[tokio::test]
async fn test_contains_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: bool = r
        .expr(["red", "green", "blue"])
        .contains("green")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    Ok(())
}
