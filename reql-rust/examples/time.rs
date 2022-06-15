use reql_rust::prelude::*;
use reql_rust::{r, Result};
use serde_json::{Value, json};
use time::macros::{date, offset, time};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let table = r.table::<Value>("users");

    table.insert(&json!({
        "id": 2,
        "full_name": "Ali",
        "date": r.now()
    })).run(&conn).await?;

    let result = table.get(1u8)
        .update(&json!({"date": r.time(date!(2022-12-01), offset!(UTC), Some(time!(12:00)))}))
        .run(&conn)
        .await?;
    
    dbg!(result);

    Ok(())
}
