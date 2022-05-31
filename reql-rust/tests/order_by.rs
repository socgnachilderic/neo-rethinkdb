use reql_rust::r;
use reql_rust::prelude::*;
use serde_json::Value;

#[tokio::test]
async fn order_by() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();
    let conn = r.connection().connect().await?;
    let user = r
        .db("rethinkdb")
        .table::<Value>("server_status")
        .order_by_key("name")
        // .with_index(r.index(r.desc("id")))
        .run(&conn)
        .await?;
        
    assert!(user.is_some());
    Ok(())
}
