use reql_rust::prelude::*;
use reql_rust::r;
use serde_json::Value;

#[tokio::test]
async fn do_query() -> reql_rust::Result<()> {
    tracing_subscriber::fmt::init();
    let conn = r.connection().connect().await?;
    let mut query = r.do_(func!(|x, y| x + y)).with_args(&[10, 20]).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    Ok(())
}
