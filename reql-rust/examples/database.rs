use reql_rust::{r, Result};
use reql_rust::types::ReadMode;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let result = r.db_list().run(&conn).await?;
    dbg!(result);

    let result = r.db_create("marvel").run(&conn).await?;
    dbg!(result);

    let result = r.db("marvel")
        .grant("bob")
        .permit_write(true)
        .permit_read(true)
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r.db("marvel").config().run(&conn).await?;
    dbg!(result);

    let result = r.db("marvel")
        .config()
        .update(json!({ "write_acks": ReadMode::Single }))
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r.db_drop("marvel").run(&conn).await?;
    dbg!(result);

    Ok(())
}
