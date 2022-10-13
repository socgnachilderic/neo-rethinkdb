use serde_json::json;

use neor::{func, r, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_filter_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_filtered: Vec<Post> = table
        .clone()
        .filter(json!({"view": 2}))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_filtered.len() == 2);
    assert!(data_filtered.first() == data.get(3));
    assert!(data_filtered.last() == data.get(1));

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_filter_data_with_func() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_filtered: Vec<Post> = table
        .clone()
        .filter(func!(|user| user.g("view").eq(r.expr(2))))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_filtered.len() == 2);
    assert!(data_filtered.first() == data.get(3));
    assert!(data_filtered.last() == data.get(1));

    tear_down(conn, &table_name).await
}
