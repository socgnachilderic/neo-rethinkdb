use reql_rust::r;
use serde_json::Value;

#[tokio::test]
async fn table() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();
    let conn = r.connection().connect().await?;
    let query = r.db("rethinkdb").table("users").run(&conn);
    let user: Option<Value> = query.await?;
    assert!(user.is_some());
    Ok(())
}
