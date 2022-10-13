use neor::arguments::DistinctOption;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_distinct_data() -> Result<()> {
    let mut data = Post::get_many_data()
        .into_iter()
        .map(|post| post.title)
        .collect::<Vec<String>>();
    data.pop();
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Vec<String> = table
        .distinct(DistinctOption::default().index("title"))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_obtained == data);

    tear_down(conn, &table_name).await
}
