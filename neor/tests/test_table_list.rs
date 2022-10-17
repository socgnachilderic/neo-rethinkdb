use neor::{r, Converter, Result};
use uuid::Uuid;

#[tokio::test]
async fn test_list_table() -> Result<()> {
    let conn = r.connection().connect().await?;
    let table_name = Uuid::new_v4().to_string();
    r.table_create(&table_name).run(&conn).await?;
    let table_list: Vec<String> = r.table_list().run(&conn).await?.unwrap().parse()?;

    assert!(table_list.len() > 0);
    r.table_drop(&table_name).run(&conn).await?;
    Ok(())
}
