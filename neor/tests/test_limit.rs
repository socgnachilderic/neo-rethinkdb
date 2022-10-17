use neor::arguments::OrderByOption;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_limit_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<Post> = table
        .order_by(OrderByOption::default().index("title"))
        .limit(1)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.first() == data.first());

    tear_down(conn, &table_name).await
}
