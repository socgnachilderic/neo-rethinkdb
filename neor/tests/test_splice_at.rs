use neor::{r, Converter, Result};

#[tokio::test]
async fn test_splice_at_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [String; 4] = r
        .expr(["Moussa", "Ali"])
        .splice_at(1, ["Fati", "Alima"])
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == ["Moussa", "Fati", "Alima", "Ali"]);

    Ok(())
}
