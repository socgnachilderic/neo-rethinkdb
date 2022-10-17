use neor::arguments::OrderByOption;
use neor::{args, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_order_by_with_opts() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<Post> = table
        .order_by(OrderByOption::default().index("id"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_order_by_title_with_opts() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let order_by_option = OrderByOption::default().index("title");
    let response: Vec<Post> = table
        .order_by(args!("id", order_by_option))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);

    tear_down(conn, &table_name).await
}
