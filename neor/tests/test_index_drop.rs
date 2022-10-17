use neor::types::IndexResponse;
use neor::{r, Converter, Result};
use uuid::Uuid;

#[tokio::test]
async fn test_drop_db() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let index_name = "author";
    let conn = r.connection().connect().await?;
    let table = r.table(table_name.as_str());

    r.table_create(&table_name).run(&conn).await?;
    table.index_create(index_name).run(&conn).await?;

    let index_dropped: IndexResponse = table
        .index_drop(index_name)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(index_dropped.dropped > Some(0));

    r.table_drop(&table_name).run(&conn).await?;
    Ok(())
}
