use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_get_data() -> Result<()> {
    let expected_post = Post::get_many_data().get(3).unwrap().to_owned();
    let (conn, table, table_name) = set_up(true).await?;
    let data_inserted: Option<Post> = table
        .get(expected_post.id)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(data_inserted == Some(expected_post));

    tear_down(conn, table_name.as_str()).await
}
