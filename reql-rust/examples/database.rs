use reql_rust::{r, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;

    let result = r.db_list().run(&conn).await?;
    dbg!(result);

    let result = r.db_create("marvel").run(&conn).await?;
    dbg!(result);

    let result = r.db("marvel").grant("bob").permit_write(true).run(&conn).await?;
    dbg!(result);

    let result = r.db_drop("marvel").run(&conn).await?;
    dbg!(result);

    Ok(())
}
