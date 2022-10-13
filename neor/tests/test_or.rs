use neor::{args, r, Converter, Result};

#[tokio::test]
async fn test_or_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let data_obtained: bool = r
        .or(args!([true, false]))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained);

    Ok(())
}
