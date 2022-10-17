use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AuthorPost {
    id: Option<u8>,
    first_name: Option<String>,
    last_name: Option<String>,
    title: Option<String>,
    content: Option<String>,
    view: Option<u8>,
}

#[tokio::test]
async fn test_union_data() -> Result<()> {
    let authors_data = json!([
        {"id": 1, "first_name": "john", "last_name": "doe"},
        {"id": 2, "first_name": "juan", "last_name": "don"},
        {"id": 3, "first_name": "jean", "last_name": "dupont"}
    ]);
    let table_name2 = Uuid::new_v4().to_string();
    let (conn, table, table_name) = set_up(true).await?;

    r.table_create(table_name2.as_str()).run(&conn).await?;
    r.table(table_name2.as_str())
        .insert(authors_data)
        .run(&conn)
        .await?;

    let response: Vec<AuthorPost> = table
        .union(r.table(table_name2.as_str()))
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(response.len() > 0);

    r.table_drop(table_name2.as_str()).run(&conn).await?;
    tear_down(conn, &table_name).await
}
