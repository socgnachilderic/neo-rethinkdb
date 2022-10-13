use neor::{r, Converter, Result};

#[tokio::test]
async fn test_json_table() -> Result<()> {
    let data = [1, 2, 3];
    let conn = r.connection().connect().await?;
    let data_obtained: [u8; 3] = r.json("[1, 2, 3]").run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained == data);

    Ok(())
}
