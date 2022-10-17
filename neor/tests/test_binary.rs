use serde::{Deserialize, Serialize};

use neor::types::Binary;
use neor::{r, Converter, Result};

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u8,
    name: String,
    avatar: Binary,
}

#[tokio::test]
async fn test_binary_ops() -> Result<()> {
    let avatar_img = std::fs::read("../logo.png")?;
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
        avatar: r.binary(&avatar_img),
    };

    let (conn, table, table_name) = set_up(false).await?;
    table.insert(&user).run(&conn).await?;
    let response: User = table.get(1).run(&conn).await?.unwrap().parse()?;

    assert!(response.id == user.id);
    assert!(response.name == user.name);
    assert!(!response.avatar.data.is_empty());

    tear_down(conn, &table_name).await
}
