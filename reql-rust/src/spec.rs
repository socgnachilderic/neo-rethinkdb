use serde::{Deserialize, Serialize};

use crate::cmd::insert::InsertOption;
use crate::types::{AnyParam, Durability};
use crate::{r, Command, Result, Session};

pub const TABLE_NAMES: [&'static str; 6] = [
    "malik",
    "malik1",
    "malik2",
    "malik3",
    "malik4",
    "malik_backup",
];

pub async fn set_up(table_name: &str, with_data: bool) -> Result<(Session, Command)> {
    let conn = r.connection().connect().await?;
    let table = r.table(table_name);

    r.table_create(table_name).run(&conn).await?;

    if with_data {
        let data = Post::get_many_data();
        let insert_option = InsertOption::default().durability(Durability::Soft);

        table.clone().index_create("title").run(&conn).await?;
        table.clone().index_wait(()).run(&conn).await?;
        table
            .clone()
            .insert((AnyParam::new(data), insert_option))
            .run(&conn)
            .await?;
    }

    Ok((conn, table))
}

pub async fn tear_down(conn: Session, table_name: &str) -> Result<()> {
    r.table_drop(table_name).run(&conn).await?;
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
            content: content.map(|text| text.to_string()),
        }
    }

    pub fn get_many_data() -> Vec<Post> {
        vec![
            Self::new(1, "title1", Some("content1"), 10),
            Self::new(2, "title2", Some("content2"), 2),
            Self::new(3, "title3", None, 0),
            Self::new(4, "title4", Some("content4"), 2),
            Self::new(5, "title4", None, 0),
        ]
    }

    pub fn get_one_data() -> Post {
        Self::new(1, "title1", Some("content1"), 0)
    }
}
