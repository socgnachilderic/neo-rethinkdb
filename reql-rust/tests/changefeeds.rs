use futures::stream::{select_all, TryStreamExt};
use reql_rust::{r, ReqlDriverError, ReqlError};
use serde_json::Value;

#[tokio::test]
async fn changefeeds_should_use_dedicated_connections() {
    tracing_subscriber::fmt::init();

    match changefeeds().await.unwrap_err() {
        ReqlError::Driver(ReqlDriverError::ConnectionLocked) => {}
        error => panic!("{:?}", error),
    }
}

async fn changefeeds() -> reql_rust::Result<()> {
    let conn = r.connection().connect().await?;

    let _ = r
        .table_create("foo")
        .run(&conn)
        .try_next()
        .await;
    let foo = r.table("foo").changes(()).run::<_, Value>(&conn);

    let _ = r
        .table_create("bar")
        .run(&conn)
        .try_next()
        .await;
    let bar = r.table("bar").changes(()).run::<_, Value>(&conn);

    let mut list = select_all(vec![foo, bar]);

    while let Some(_) = list.try_next().await? {}

    Ok(())
}
