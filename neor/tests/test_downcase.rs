use neor::{r, Converter, Result};

#[tokio::test]
async fn test_downcase_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: String = r
        .expr("Sentence about LaTeX.")
        .downcase()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == "sentence about latex.");

    Ok(())
}
