use reql_rust::{r, Result};
use serde_json::{Value, json};
use time::macros::{date, offset, time};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let table = r.table::<Value>("users");

    table.insert(&json!({
        "full_name": "Ali",
        "date": r.now()
    })).run(&conn).await?;

    table.insert(&json!({
        "full_name": "Abomo",
        "date": r.time(date!(2022-12-01), offset!(UTC), Some(time!(12:00)))
    })).run(&conn).await?;

    table.insert(&json!({
        "full_name": "Malika",
        "date": r.epoch_time(531360000)?
    })).run(&conn).await?;

    table.insert(&json!({
        "full_name": "Abdoul",
        "date": r.iso8601("1986-11-03T08:30:00-07:00", None)?
    })).run(&conn).await?;

    table.insert(&json!({
        "full_name": "Fati",
        "date": r.iso8601("1986-11-03T08:30:00", Some(offset!(+07:00)))?
    })).run(&conn).await?;

    let result = r.now().in_timezone(offset!(-08:00)).timezone();
    dbg!(result);

    Ok(())
}
