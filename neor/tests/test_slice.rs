use neor::arguments::OrderByOption;
use neor::{args, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_slice_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Vec<Post> = table
        .order_by(OrderByOption::default().index("id"))
        .slice(args!(4, 5))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained.last() == data.last());

    tear_down(conn, &table_name).await
}
