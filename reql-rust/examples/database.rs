use reql_rust::prelude::*;
use reql_rust::{r, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let result = r.db_list().run(&conn).try_next().await?;
    dbg!(result);

    let result = r.db_create("marvel").run(&conn).try_next().await?;
    dbg!(result);

    let result = r.db_drop("marvel").run(&conn).try_next().await?;
    dbg!(result);

    Ok(())
}
