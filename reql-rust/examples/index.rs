use reql_rust::{r, Result, Session};
use reql_rust::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;
    
    set_up(&conn).await?;
    conn.use_("marvel").await;
    
    let result = r.table::<serde_json::Value>("heroes")
        .index_create("mail")
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r
        .table::<serde_json::Value>("heroes")
        .index_create("author_name")
        .with_func(func!(|row| row.bracket("author").bracket("name")))
        .with_geo(true)
        .with_multi(true)
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r
        .table::<serde_json::Value>("heroes")
        .index_status()
        .with_indexes(&vec!["author_name", "mail"])
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r
        .table::<serde_json::Value>("heroes")
        .index_wait()
        .with_one_index("mail")
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r
        .table::<serde_json::Value>("heroes")
        .index_rename("author_name", "code_name")
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r.table::<serde_json::Value>("heroes").index_list().run(&conn).await?;
    dbg!(result);

    let result = r
        .table::<serde_json::Value>("heroes")
        .index_drop("code_name")
        .run(&conn)
        .await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    r.db_create("marvel").run(conn).await?;
    r.db("marvel")
        .table_create("heroes")
        .run(conn)
        .await?;

    Ok(())
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.table::<serde_json::Value>("heroes")
        .index_drop("mail")
        .run(conn)
        .await?;
    r.table_drop("heroes").run(conn).await?;
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
