// FIX Not working
use std::ops::Add;

use neor::{func, r, Result};

use common::*;

mod common;

#[tokio::test]
#[ignore = "not work"]
async fn test_do_opts() -> Result<()> {
    let (conn, table, table_name) = set_up(true).await?;
    let response = table
        .get(1)
        .do_(func!(|post| post.g("view").add(r.expr(5))))
        .run(&conn)
        .await;

    dbg!(&response);

    tear_down(conn, &table_name).await
}
