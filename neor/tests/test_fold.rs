use neor::{args, func, r, Converter, Result};

use common::*;

mod common;

#[tokio::test]
async fn test_fold_ops() -> Result<()> {
    let posts = Post::get_many_data()
        .into_iter()
        .fold(String::new(), |acc, post| {
            format!("{}{}{}", acc, if acc == "" { "" } else { ", " }, post.title)
        });
    let (conn, table, table_name) = set_up(true).await?;
    let response: String = table
        .order_by("id")
        .fold(
            "",
            func!(|acc, post| acc.clone()
                + r.branch(acc.eq(""), args!("", ", "))
                + post.g("title")),
        )
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == posts);

    tear_down(conn, &table_name).await
}
