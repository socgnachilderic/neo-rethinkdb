use neor::types::IndexStatusResponse;
use neor::{args, Command, Converter, Result, Session};

use common::{set_up, tear_down};

mod common;

#[tokio::test]
async fn test_get_index_waited() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_data(&conn, &table).await?;

    let indexes_waited: Vec<IndexStatusResponse> =
        table.index_wait(()).run(&conn).await?.unwrap().parse()?;

    assert!(indexes_waited.len() == 3);
    indexes_waited
        .iter()
        .for_each(|index_waited| assert!(index_waited.ready));

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_get_index_status_with_param() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_data(&conn, &table).await?;

    let index_waited = table
        .index_wait("author")
        .run(&conn)
        .await?
        .unwrap()
        .parse::<Vec<IndexStatusResponse>>()?;

    let index_waited = index_waited.first().unwrap();

    assert!(index_waited.index == "author");
    assert!(index_waited.ready);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_get_index_status_with_params() -> Result<()> {
    let (conn, table, table_name) = set_up(false).await?;
    generate_data(&conn, &table).await?;

    let indexes_waited: Vec<IndexStatusResponse> = table
        .index_wait(args!(["age", "name"]))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(indexes_waited.len() == 2);
    indexes_waited
        .iter()
        .for_each(|index_waited| assert!(index_waited.ready));

    tear_down(conn, &table_name).await
}

async fn generate_data(conn: &Session, table: &Command) -> Result<()> {
    table.index_create("author").run(conn).await?;
    table.index_create("name").run(conn).await?;
    table.index_create("age").run(conn).await?;
    Ok(())
}
