use reql_rust::{r, Result, Session};
use reql_rust::prelude::*;
use serde::{Serialize, Deserialize};

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
        .run(&conn)
        .try_next()
        .await?;
    dbg!(result);
    
    let result = r.table("heroes")
        .insert(&posts)
        .run(&conn)
        .try_next()
        .await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    r.db_create("marvel").run(conn).try_next().await?;
    r.db("marvel")
        .table_create("heroes")
        .run(conn)
        .try_next()
        .await?;

    Ok(())
}

async fn tear_down(conn: &Session) -> Result<()> {
    r.table_drop("heroes").run(conn).try_next().await?;
    r.db_drop("marvel").run(conn).try_next().await?;

    Ok(())
}
