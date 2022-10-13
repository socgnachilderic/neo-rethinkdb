use std::collections::HashMap;

use neor::{r, Result, Converter};

use common::Post;

mod common;

#[tokio::test]
async fn test_hash_map_ops() -> Result<()> {
    let expected_post = Post::get_one_data();
    let conn = r.connection().connect().await?;
    let mut post = HashMap::new();
    
    post.insert("id", r.expr(&expected_post.id));
    post.insert("title", r.expr(&expected_post.title));
    post.insert("content", r.expr(&expected_post.content));
    post.insert("view", r.expr(&expected_post.view));

    let response: Post = r.hash_map(post).run(&conn).await?.unwrap().parse()?;

    assert_eq!(response, expected_post);

    Ok(())
}
