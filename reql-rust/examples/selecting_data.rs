use reql_rust::prelude::*;
use reql_rust::{r, Result, Session};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Users {
    id: u8,
    full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Posts {
    id: u8,
    title: String,
    content: String,
    user_id: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = r.connection().connect().await?;
    set_up(&conn).await?;
    conn.use_("marvel").await;

    let user_table = r.db("marvel").table::<Users>("users");
    let post_table = r.db("marvel").table::<Posts>("posts");

    let result = post_table.run(&conn).await?;
    dbg!(result);

    let result = post_table.get(2).run(&conn).await?;
    dbg!(result);

    // let result = post_table
    //     .get_all(&["title"])
    //     .run(&conn)
    //     .await?;
    // dbg!(result);

    let result = post_table.between(1, 4).run(&conn).await?;
    dbg!(result);

    let result = post_table
        .filter(func!(|row| row.bracket("id").eq(3)))
        .run(&conn)
        .await?;
    dbg!(result);

    let result = post_table
        .inner_join(
            &user_table,
            func!(|post, _user| post.bracket("user_id").eq(1)),
        )
        .run(&conn)
        .await?;
    dbg!(result);

    let result = post_table
        .outer_join(
            &user_table,
            func!(|post, _user| post.bracket("user_id").eq(1)),
        )
        .zip()
        .run(&conn)
        .await?;
    dbg!(result);

    let result = post_table
        .eq_join("user_id", &user_table)
        .with_ordered(true)
        .run(&conn)
        .await?;
    dbg!(result);

    tear_down(&conn).await?;

    Ok(())
}

async fn set_up(conn: &Session) -> Result<()> {
    let users = vec![
        Users {
            id: 1,
            full_name: "John Doe".to_string(),
        },
        Users {
            id: 2,
            full_name: "Don Juan".to_string(),
        },
    ];

    let posts = vec![
        Posts {
            id: 1,
            title: "title 1".to_string(),
            content: "content 1".to_string(),
            user_id: 1,
        },
        Posts {
            id: 2,
            title: "title 2".to_string(),
            content: "content 2".to_string(),
            user_id: 2,
        },
        Posts {
            id: 3,
            title: "title 3".to_string(),
            content: "content 3".to_string(),
            user_id: 1,
        },
        Posts {
            id: 4,
            title: "title 4".to_string(),
            content: "content 4".to_string(),
            user_id: 2,
        },
        Posts {
            id: 5,
            title: "title 5".to_string(),
            content: "content 5".to_string(),
            user_id: 1,
        },
    ];

    r.db_create("marvel").run(conn).await?;
    r.db("marvel").table_create("users").run(conn).await?;
    r.db("marvel").table_create("posts").run(conn).await?;

    r.db("marvel")
        .table::<Posts>("posts")
        .index_create("title")
        .run(conn)
        .await?;

    r.db("marvel")
        .table("users")
        .insert(&users)
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
    r.db_drop("marvel").run(conn).await?;

    Ok(())
}
