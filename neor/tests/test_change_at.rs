use neor::{r, Converter, Result};

#[tokio::test]
async fn test_change_at_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [String; 3] = r
        .expr(["Moussa", "Ali", "Fati"])
        .change_at(1, "Alima")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == ["Moussa", "Alima", "Fati"]);

    Ok(())
}
