use neor::arguments::CoerceType;
use neor::{r, Converter, Result};
use serde_json::json;

use common::Post;

mod common;

#[tokio::test]
async fn test_coerce_to_ops() -> Result<()> {
    let data = Post::get_one_data();
    let conn = r.connection().connect().await?;
    let response: Post = r
        .expr(json!([
            ["id", 1],
            ["title", "title1"],
            ["content", "content1"],
            ["view", 0]
        ]))
        .coerce_to(CoerceType::Object)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    let response2: char = r
        .expr(1)
        .coerce_to(CoerceType::String)
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response == data);
    assert!(response2 == '1');

    Ok(())
}
