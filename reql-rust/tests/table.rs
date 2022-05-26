use reql_rust::r;
use serde_json::Value;

#[tokio::test]
async fn table() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();
    let conn = r.connection().connect().await?;
    let user = r.db("rethinkdb").table::<Value>("users").run(&conn).await?;
    assert!(user.is_some());
    Ok(())
}
