use reql_rust::cmd::table::TableBuilder;
use reql_rust::prelude::*;
use reql_rust::{r, types::ReturnChanges, Result, Session};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = r.connection().connect().await?;
    let my_table = set_up(&conn).await?;

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

    let result = my_table.insert(&post_1).run(&conn).await?;
    dbg!(result);

    let result = my_table.insert_many(&posts).run(&conn).await?;
    dbg!(result);

    let result = my_table
        .update(&json!({ "status": "published" }))
        .with_return_changes(ReturnChanges::Bool(true))
        .run(&conn)
        .await?;
    dbg!(result);

    /* let result = my_table
        .replace_by_func(func!(|post| post.without("status")))
        .with_return_changes(ReturnChanges::Bool(true))
        .run(&conn).await?;
    dbg!(result); */

    let result = my_table.delete().run(&conn).await?;
    dbg!(result);

    let result = my_table.sync().run(&conn).await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<TableBuilder<Posts>> {
    let my_table = r.db("marvel").table::<Posts>("posts");
    r.db_create("marvel").run(conn).await?;
    r.db("marvel").table_create("posts").run(conn).await?;

    Ok(my_table)
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
