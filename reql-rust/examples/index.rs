use reql_rust::cmd::table::TableBuilder;
use reql_rust::prelude::*;
use reql_rust::{r, Result, Session};

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;
    let my_table = set_up(&conn).await?;

    let result = my_table.index_create("mail").run(&conn).await?;
    dbg!(result);

    let result = my_table
        .index_create("author_name")
        .with_func(func!(|row| row.bracket("author").bracket("name")))
        .with_geo(true)
        .with_multi(true)
        .run(&conn)
        .await?;
    dbg!(result);

    let result = my_table
        .index_status()
        .with_indexes(&vec!["author_name", "mail"])
        .run(&conn)
        .await?;
    dbg!(result);

    let result = my_table
        .index_wait()
        .with_one_index("mail")
        .run(&conn)
        .await?;
    dbg!(result);

    let result = my_table
        .index_rename("author_name", "code_name")
        .run(&conn)
        .await?;
    dbg!(result);

    let result = my_table.index_list().run(&conn).await?;
    dbg!(result);

    let result = my_table.index_drop("code_name")
        .run(&conn)
        .await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<TableBuilder<serde_json::Value>> {
    let my_table = r.db("marvel").table::<serde_json::Value>("heroes");
    r.db_create("marvel").run(conn).await?;
    r.db("marvel").table_create("heroes").run(conn).await?;

    Ok(my_table)
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
