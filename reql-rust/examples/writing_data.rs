use reql_rust::{r, Result, Session, types::ReturnChanges};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    // id: u64,
    title: &'static str,
    content: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;
    let post_1 = Posts {
        // id: 1,
        title: "Lorem ipsum",
        content: "Dolor sit amet",
    };

    let post_2 = Posts {
        title: "title 1",
        content: "content 1",
    };

    let post_3 = Posts {
        title: "title 2",
        content: "content 2",
    };

    let posts = vec![&post_2, &post_3];
    
    set_up(&conn).await?;
    conn.use_("marvel").await;
    
    let result = r.table("heroes")
        .insert(&post_1)
        .run(&conn).await?;
    dbg!(result);
    
    let result = r.table("heroes")
        .insert(&posts)
        .run(&conn).await?;
    dbg!(result);

    let result = r.table("heroes")
        .update(&json!({ "status": "published" }))
        .with_return_changes(ReturnChanges::Bool(true))
        .run(&conn).await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    r.db_create("marvel").run(conn).await?;
    r.db("marvel")
        .table_create("heroes")
        .run(conn).await?;

    Ok(())
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.table_drop("heroes").run(conn).await?;
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
