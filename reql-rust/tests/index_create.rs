use futures::TryStreamExt;
use reql_rust::{func, r};
use serde_json::Value;

#[tokio::test]
async fn index_create() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();

    let conn = r.connect(()).await?;

    let _ = r
        .table_create("comments")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_drop("author_name")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_create(r.args((
            "author_name",
            func!(|doc| doc.bracket("author").bracket("name")),
        )))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    let _ = r
        .table("comments")
        .index_drop("post_and_date")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_create(r.args((
            "post_and_date",
            func!(|doc| [doc.clone().bracket("post_id"), doc.bracket("date")]),
        )))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    Ok(())
}
