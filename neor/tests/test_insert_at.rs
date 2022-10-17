use neor::{r, Converter, Result};

#[tokio::test]
async fn test_insert_at_ops() -> Result<()> {
    let conn = r.connection().connect().await?;
    let response: [String; 4] = r
        .expr(["Moussa", "Ali", "Fati"])
        .insert_at(1, "Alima")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == ["Moussa", "Alima", "Ali", "Fati"]);

    Ok(())
}
