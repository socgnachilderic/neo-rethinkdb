#![allow(dead_code)]

use neor::arguments::{Durability, InsertOption};
use neor::{args, r, Command, Result, Session};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub async fn set_up(with_data: bool) -> Result<(Session, Command, String)> {
    let table_name = Uuid::new_v4().to_string();
    let user_rethinkdb = json!({
        "id": "malik",
        "password": "malik"
    });
    let conn = r.connection().connect().await?;
    let table = r.table(table_name.as_str());

    r.table_create(table_name.as_str()).run(&conn).await?;
    r.db("rethinkdb")
        .table("users")
        .insert(user_rethinkdb)
        .run(&conn)
        .await?;

    if with_data {
        let data = Post::get_many_data();
        let insert_option = InsertOption::default().durability(Durability::Soft);

        table.index_create("title").run(&conn).await?;
        table.index_wait(()).run(&conn).await?;
        table.insert(args!(data, insert_option)).run(&conn).await?;
    }

    Ok((conn, table, table_name))
}

pub async fn tear_down(conn: Session, table_name: &str) -> Result<()> {
    r.table_drop(table_name).run(&conn).await?;
    r.db("rethinkdb")
        .table("users")
        .get("malik")
        .delete(())
        .run(&conn)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub id: u8,
    pub title: String,
    pub content: Option<String>,
    pub view: u8,
}

impl Post {
    pub fn new(id: u8, title: &str, content: Option<&str>, view: u8) -> Self {
        Self {
            id,
            view,
            title: title.to_string(),
            content: content.map(String::from),
        }
    }

    pub fn get_many_data() -> Vec<Self> {
        vec![
            Self::new(1, "title1", Some("content1"), 10),
            Self::new(2, "title2", Some("content2"), 2),
            Self::new(3, "title3", None, 0),
            Self::new(4, "title4", Some("content4"), 2),
            Self::new(5, "title4", None, 0),
        ]
    }

    pub fn get_one_data() -> Self {
        Self::new(1, "title1", Some("content1"), 0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comment {
    pub id: u8,
    pub text: String,
    pub post_id: u8,
}

impl Comment {
    pub fn new(id: u8, content: &str, post_id: u8) -> Self {
        Self {
            id,
            post_id,
            text: content.to_string(),
        }
    }

    pub fn get_many_data() -> Vec<Self> {
        vec![
            Self::new(1, "comment1", 1),
            Self::new(2, "comment2", 2),
            Self::new(3, "comment3", 3),
            Self::new(4, "comment4", 2),
            Self::new(5, "comment4", 1),
        ]
    }

    pub async fn own_set_up() -> Result<(Session, Command, Command, String, String)> {
        let comment_tablename = Uuid::new_v4().to_string();
        let (conn, post_table, post_tablename) = set_up(true).await?;
        let comment_table = r.table(&comment_tablename);

        r.table_create(comment_tablename.as_str())
            .run(&conn)
            .await?;
        comment_table
            .insert(Self::get_many_data())
            .run(&conn)
            .await?;

        Ok((
            conn,
            comment_table,
            post_table,
            comment_tablename,
            post_tablename,
        ))
    }

    pub async fn own_tear_down(
        conn: Session,
        comment_tablename: String,
        post_tablename: String,
    ) -> Result<()> {
        r.table_drop(&comment_tablename).run(&conn).await?;
        tear_down(conn, &post_tablename).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommentWithPost {
    pub id: u8,
    pub text: String,
    pub post_id: u8,
    pub title: String,
    pub content: Option<String>,
    pub view: u8,
}
