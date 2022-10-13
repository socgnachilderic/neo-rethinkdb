use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_avg_data() -> Result<()> {
    let data: Vec<u8> = Post::get_many_data().iter().map(|post| post.view).collect();
    let avg = data.iter().sum::<u8>() as f32 / data.len() as f32;
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: f32 = table.avg("view").run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained == avg);

    tear_down(conn, table_name.as_str()).await
}
