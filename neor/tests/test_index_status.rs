use neor::types::IndexStatusResponse;
use neor::{args, Command, Converter, Result, Session};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_get_index_status() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_index(&conn, &table).await?;

    let index_status: Vec<IndexStatusResponse> =
        table.index_status(()).run(&conn).await?.unwrap().parse()?;

    assert!(index_status.len() == 3);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_get_index_status_with_param() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_index(&conn, &table).await?;

    let index_status: Vec<IndexStatusResponse> = table
        .index_status("author")
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(index_status.len() == 1);
    assert!(index_status.first().unwrap().index == "author");

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_get_index_status_with_params() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_index(&conn, &table).await?;

    let index_status: Vec<IndexStatusResponse> = table
        .index_status(args!(["age", "name"]))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(index_status.len() == 2);
    assert!(index_status.first().unwrap().index == "age");
    assert!(index_status.last().unwrap().index == "name");

    tear_down(conn, &table_name).await
}

async fn generate_index(conn: &Session, table: &Command) -> Result<()> {
    table.index_create("author").run(conn).await?;
    table.index_create("name").run(conn).await?;
    table.index_create("age").run(conn).await?;

    Ok(())
}
