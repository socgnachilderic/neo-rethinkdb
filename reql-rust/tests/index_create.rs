use reql_rust::prelude::*;
use reql_rust::r;

#[tokio::test]
async fn index_create() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();

    let conn = r.connection().connect().await?;

    let _ = r
        .table_create("comments")
        .run(&conn)
        .await;

    let _ = r
        .table::<serde_json::Value>("comments")
        .index_drop("author_name")
        .run(&conn)
        .await;

    let _ = r
        .table::<serde_json::Value>("comments")
        .index_create("author_name")
        .with_func(func!(|doc| doc.bracket("author").bracket("name")))
        .run(&conn)
        .await?;

    let _ = r
        .table::<serde_json::Value>("comments")
        .index_drop("post_and_date")
        .run(&conn)
        .await;

    let _ = r
        .table::<serde_json::Value>("comments")
        .index_create("post_and_date")
        .with_func(func!(|doc| [doc.clone().bracket("post_id"), doc.bracket("date")]))
        .run(&conn)
        .await?;

    Ok(())
}
