use reql_rust::prelude::*;
use reql_rust::{r, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;
    // let result = r.table_create("foo")
    //     .with_primary_key("_id")
    //     .with_shards(2)
    //     .run(&conn)
    //     .try_next().await?;

    let result = r.table_drop("foo").run(&conn).try_next().await?;

    dbg!(result);
    Ok(())
}
