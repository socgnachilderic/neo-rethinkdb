use neor::types::MutationResponse;
use neor::{func, Converter, Result};

use common::*;

mod common;

#[tokio::test]
async fn test_replace_docs() -> Result<()> {
    let data = Post::get_one_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table
        .get(1)
        .replace(data)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.replaced == 1);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_replace_docs_with_func() -> Result<()> {
    let lenght = Post::get_many_data().len();
    let (conn, table, table_name) = set_up(true).await?;
    let response: MutationResponse = table
        .replace(func!(|post| post.without("view")))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.replaced == lenght);

    tear_down(conn, &table_name).await
}
