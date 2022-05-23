use reql_rust::{r, ReqlDriverError, ReqlError};

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
        .await;
    // let foo = r.table("foo").changes().run::<_, Value>(&conn);

    let _ = r
        .table_create("bar")
        .run(&conn)
        .await;
    // let bar = r.table("bar").changes().run::<_, Value>(&conn);

    // let mut list = select_all(vec![foo, bar]);

    // while let Some(_) = list.await? {}

    Ok(())
}
