use neor::arguments::{ReadMode, TableOption};
use neor::{args, r, Converter, Result};
use serde_json::Value;

#[tokio::test]
async fn test_table() -> Result<()> {
    let conn = r.connection().connect().await?;
    let table: Vec<Value> = r
        .db("todo_app")
        .table("geo")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(table.len() > 0);
    Ok(())
}

#[tokio::test]
async fn test_table_with_options() -> Result<()> {
    let conn = r.connection().connect().await?;
    let table_options = TableOption::default().read_mode(ReadMode::Outdated);
    let table: Vec<Value> = r
        .db("todo_app")
        .table(args!("geo", table_options))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(table.len() > 0);
    Ok(())
}
