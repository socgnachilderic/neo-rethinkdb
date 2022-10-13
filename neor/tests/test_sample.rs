use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_sample_data() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let data_obtained: Vec<Post> = table.sample(3).run(&conn).await?.unwrap().parse()?;

    assert!(data_obtained.len() == 3);

    tear_down(conn, &table_name).await
}
