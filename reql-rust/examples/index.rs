use reql_rust::prelude::*;
use reql_rust::{r, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;
    r.db_create("marvel").run(&conn).try_next().await?;
    r.db("marvel").table_create("heroes").run(&conn).try_next().await?;

    conn.use_("marvel").await;

    let result = r.table("heroes").index_create("author_name").run::<_, serde_json::Value>(&conn).try_next().await?;
    dbg!(result);

    let result = r.table("heroes").index_drop("author_name").run(&conn).try_next().await?;
    dbg!(result);

    r.table_drop("heroes").run(&conn).try_next().await?;
    r.db_drop("marvel").run(&conn).try_next().await?;
    Ok(())
}