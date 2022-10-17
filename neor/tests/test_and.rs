use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_and_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: bool = r
        .and(args!([true, true, true]))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response);

    Ok(())
}
