use reql_rust::{r, Result, Session};
use reql_rust::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    id: u8,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;    
    set_up(&conn).await?;
    conn.use_("marvel").await;
    
    let result = r.db("marvel")
        .table::<serde_json::Value>("posts")
        .run(&conn).await?;
    dbg!(result);
    
    let result = r.table::<Posts>("posts")
        .get(2)
        // .changes()
        .run(&conn)
        .await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    let posts = vec![
        Posts { id: 1, title: "title 1".to_string(), content: "content 1".to_string() },
        Posts { id: 2, title: "title 2".to_string(), content: "content 2".to_string() },
        Posts { id: 3, title: "title 3".to_string(), content: "content 3".to_string() },
        Posts { id: 4, title: "title 4".to_string(), content: "content 4".to_string() },
        Posts { id: 5, title: "title 5".to_string(), content: "content 5".to_string() },
    ];

    r.db_create("marvel").run(conn).await?;
    r.db("marvel")
        .table_create("posts")
        .run(conn)
        .await?;
    r.db("marvel")
        .table("posts")
        .insert(&posts)
        .run(conn)
        .await?;

    Ok(())
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.table_drop("posts").run(conn).await?;
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
