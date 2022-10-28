use neor::{args, r, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_slice_data() -> Result<()> {
    let data = Post::get_many_data();
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<Post> = table
        .order_by(r.index("id"))
        .slice(args!(4, 5))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.last() == data.last());

    tear_down(conn, &table_name).await
}
