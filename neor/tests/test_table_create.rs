use neor::arguments::TableCreateOption;
use neor::types::DbResponse;
use neor::{args, Converter, Session};
use neor::{r, Result};
use uuid::Uuid;

#[tokio::test]
async fn test_create_table() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let table_created: DbResponse = r
        .table_create(table_name.as_str())
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    drop_table(&table_name, table_created, &conn).await
}

#[tokio::test]
async fn test_create_table_with_options() -> Result<()> {
    let table_name = Uuid::new_v4().to_string();
    let conn = r.connection().connect().await?;
    let table_options = TableCreateOption::default().primary_key("id");
    let table_created = r
        .db("test")
        .table_create(args!(table_name.as_str(), table_options))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    drop_table(&table_name, table_created, &conn).await
}

async fn drop_table(table_name: &str, table_created: DbResponse, conn: &Session) -> Result<()> {
    assert!(table_created.tables_created > Some(0));
    r.table_drop(table_name).run(conn).await?;
    Ok(())
}
