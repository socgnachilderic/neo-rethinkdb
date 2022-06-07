use reql_rust::{r, Result};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let result = r.db("test").table_list().run(&conn).await?;
    dbg!(result);

    let result = r.db("test").table_create("foo").run(&conn).await?;
    dbg!(result);

    let result = r.db("test").table::<Value>("foo").rebalance().run(&conn).await?;
    dbg!(result);

    let result = r.db("test")
        .table::<Value>("foo")
        .reconfigure()
        .with_shards(2)
        .with_replicas(reql_rust::types::Replicas::Int(1))
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r.db("test").table_drop("foo").run(&conn).await?;
    dbg!(result);

    Ok(())
}
