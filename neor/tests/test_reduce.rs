use neor::{func, r, Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_reduce_ops() -> Result<()> {
    let post_number = Post::get_many_data().len();
    let (conn, table, table_name) = set_up(true).await?;
    let response: usize = table
        .map(func!(|| r.expr(1)))
        .reduce(func!(|left, right| left + right))
        .default(0)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == post_number);

    tear_down(conn, &table_name).await
}
