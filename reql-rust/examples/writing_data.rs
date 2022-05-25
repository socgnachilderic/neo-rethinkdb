use reql_rust::{r, Result, Session, types::ReturnChanges};
use reql_rust::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;
    let post_1 = Posts {
        title: "Lorem ipsum".to_string(),
        content: "Dolor sit amet".to_string(),
    };

    let post_2 = Posts {
        title: "title 1".to_string(),
        content: "content 1".to_string(),
    };

    let post_3 = Posts {
        title: "title 2".to_string(),
        content: "content 2".to_string(),
    };

    let posts = vec![post_2, post_3];
    
    set_up(&conn).await?;
    conn.use_("marvel").await;
    
    let result = r.table("posts")
        .insert(&post_1)
        .run(&conn).await?;
    dbg!(result);
    
    let result = r.table("posts")
        .insert(&posts)
        .run(&conn).await?;
    dbg!(result);

    let result = r.table::<Posts>("posts")
        .update(&json!({ "status": "published" }))
        .with_return_changes(ReturnChanges::Bool(true))
        .run(&conn).await?;
    dbg!(result);

    /* let result = r.table("heroes")
        .replace_by_func(func!(|post| post.without("status")))
        .with_return_changes(ReturnChanges::Bool(true))
        .run(&conn).await?;
    dbg!(result); */

    let result = r.table::<Posts>("posts")
        .delete()
        .run(&conn)
        .await?;
    dbg!(result);

    let result = r.table::<Posts>("posts").sync().run(&conn).await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    r.db_create("marvel").run(conn).await?;
    r.db("marvel")
        .table_create("posts")
        .run(conn).await?;

    Ok(())
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.table_drop("posts").run(conn).await?;
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
