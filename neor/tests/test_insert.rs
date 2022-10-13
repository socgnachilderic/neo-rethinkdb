use neor::arguments::{InsertOption, ReturnChanges};
use neor::types::MutationResponse;
use neor::{args, r, Converter, Result};
use uuid::Uuid;

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_insert_data() -> Result<()> {
    let data = Post::get_one_data();
    let (conn, table, table_name) = set_up(false).await?;
    let data_inserted: MutationResponse = table
        .clone()
        .insert(&data)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_inserted.inserted == 1);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_insert_many_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(false).await?;
    let data_inserted: MutationResponse = table
        .clone()
        .insert(&data)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_inserted.inserted == data.len());

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_insert_data_by_copy() -> Result<()> {
    let data = Post::get_many_data();
    let table_name2 = Uuid::new_v4().to_string();
    let (conn, table, table_name) = set_up(false).await?;

    r.table_create(table_name2.as_str()).run(&conn).await?;
    table.insert(&data).run(&conn).await?;

    let data_inserted: MutationResponse = r
        .table(table_name2.as_str())
        .insert(table)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_inserted.inserted == data.len());

    r.table_drop(table_name2.as_str()).run(&conn).await?;
    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_insert_data_with_opts() -> Result<()> {
    let data = Post::get_one_data();
    let (conn, table, table_name) = set_up(false).await?;
    let insert_options = InsertOption::default().return_changes(ReturnChanges::Bool(true));
    let data_inserted: MutationResponse = table
        .clone()
        .insert(args!(&data, insert_options))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!((&data_inserted).inserted == 1);
    let expected_data: Post = data_inserted
        .changes
        .unwrap()
        .first()
        .unwrap()
        .clone()
        .new_val
        .unwrap()
        .parse()?;
    assert!(expected_data == data);

    tear_down(conn, &table_name).await
}
