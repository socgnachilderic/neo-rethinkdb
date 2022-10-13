use neor::arguments::OrderByOption;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_nth_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Post = table
        .order_by(OrderByOption::default().index("title"))
        .nth(-1)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data.last() == Some(&data_obtained));

    tear_down(conn, &table_name).await
}
