use futures::TryStreamExt;
use reql_rust::{func, r};
use serde_json::Value;

#[tokio::test]
async fn do_query() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();
    let conn = r.connection().connect().await?;
    let mut query = r.do_(r.args(([10, 20], func!(|x, y| x + y)))).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    Ok(())
}
