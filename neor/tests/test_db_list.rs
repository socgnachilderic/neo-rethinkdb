use neor::{r, Converter, Result};

#[tokio::test]
async fn test_list_db() -> Result<()> {
    let conn = r.connection().connect().await?;
    let db_list: Vec<String> = r.db_list().run(&conn).await?.unwrap().parse()?;

    assert!(db_list.len() > 0);
    Ok(())
}
