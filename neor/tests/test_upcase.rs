use neor::{r, Converter, Result};

#[tokio::test]
async fn test_upcase_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: String = r
        .expr("Sentence about LaTeX.")
        .upcase()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == "SENTENCE ABOUT LATEX.");

    Ok(())
}
