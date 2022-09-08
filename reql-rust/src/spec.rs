use serde::{Deserialize, Serialize};

use crate::prelude::Document;
use crate::{r, Command, Result, Session};

pub const DATABASE_NAMES: [&'static str; 6] = [
    "malik",
    "malik1",
    "malik2",
    "malik3",
    "malik4",
    "malik_backup",
];

pub async fn set_up(table_name: &str) -> Result<(Session, Command)> {
    let conn = r.connection().connect().await?;
    let table = r.table(table_name);
    r.table_create(table_name).run(&conn).await?;
    table.clone().run(&conn).await?;

    Ok((conn, table))
}

pub async fn tear_down(conn: Session, table_name: &str) -> Result<()> {
    r.table_drop(table_name).run(&conn).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Document)]
pub struct Post {
    pub title: String,
    pub content: Option<String>,
}

impl Post {
    pub fn new(title: &str, content: Option<&str>) -> Self {
        Self {
            title: title.to_string(),
            content: content.map(|text| text.to_string()),
        }
    }

    pub fn get_many_data() -> Vec<Post> {
        vec![
            Self::new("title1", Some("content1")),
            Self::new("title2", Some("content2")),
            Self::new("title3", None),
            Self::new("title4", Some("content4")),
        ]
    }

    pub fn get_one_data() -> Post {
        Self::new("title1", Some("content1"))
    }
}
