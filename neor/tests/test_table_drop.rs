use neor::types::DbResponse;
use neor::{r, Converter, Result};
use uuid::Uuid;

#[tokio::test]
async fn test_drop_table() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;

    r.table_create(table_name.as_str()).run(&conn).await?;

    let table_dropped: DbResponse = r
        .table_drop(&table_name)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(table_dropped.tables_dropped > Some(0));

    Ok(())
}
