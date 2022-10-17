use neor::{args, r, Converter, Result};

use common::*;

mod common;

#[tokio::test]
async fn test_desc_ops() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response: Vec<Post> = table
        .order_by(args!([r.expr("view"), r.desc("title")]))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.len() > 1);

    tear_down(conn, &table_name).await
}
