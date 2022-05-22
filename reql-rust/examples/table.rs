use reql_rust::prelude::*;
use reql_rust::{r, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let result = r.db("test").table_list().run(&conn).try_next().await?;
    dbg!(result);

    let result = r
        .db("test")
        .table_create("foo")
        .run(&conn)
        .try_next()
        .await?;
    dbg!(result);

    let result = r.db("test").table_drop("foo").run(&conn).try_next().await?;
    dbg!(result);

    Ok(())
}
