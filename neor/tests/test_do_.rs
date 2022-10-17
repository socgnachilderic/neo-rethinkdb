use neor::{args, func, r, Converter, Result};

use common::*;

mod common;

#[tokio::test]
async fn test_do_ops() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: u8 = table
        .get(1)
        .do_(func!(|post| post.g("view") + 5))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response, 15);

    tear_down(conn, &table_name).await
}

#[tokio::test]
async fn test_do_ops_with_array() -> Result<()> {
    let data = Post::get_many_data().get(1).map(ToOwned::to_owned);
    let (conn, table, table_name) = set_up(true).await?;
    let response: Option<Post> = r
        .do_(
            [table.get(1), table.get(2)],
            func!(|post1, post2| {
                r.branch(post1.g("view").lt(post2.g("view")), args!(post1, post2))
            }),
        )
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert_eq!(response, data);

    tear_down(conn, &table_name).await
}
