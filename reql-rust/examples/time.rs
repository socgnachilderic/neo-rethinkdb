use reql_rust::{r, Result};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let table = r.table::<Value>("users");

    table.insert(&json!({
        "full_name": "Juan",
        "date": r.now(),
    })).run(&conn).await?;

    Ok(())
}
