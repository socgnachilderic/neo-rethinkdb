// FIX Not working
use std::collections::HashMap;

use neor::{r, Result};
use serde_json::json;

use common::{set_up, tear_down};

mod common;

#[tokio::test]
#[ignore = "not work"]
async fn test_literal_ops() -> Result<()> {
    let data = json!([
        {
            "id": 1,
            "name": "Alima",
            "data": {
                "age": 18,
                "city": "Dakar"
            }
        },
        {
            "id": 2,
            "name": "Ibrahim",
            "data": {
                "age": 21,
                "city": "Garoua"
            }
        },
    ]);

    let mut dt = HashMap::new();
    dt.insert(
        "data",
        r.literal(json!({
            "age": 19,
            "job": "Engineer"
        })),
    );

    let (conn, table, table_name) = set_up(false).await?;
    table.insert(data).run(&conn).await?;
    let response = r
        .table(&table_name)
        .get(1)
        .update(r.hash_map(dt))
        .run(&conn)
        .await?;

    dbg!(&response);

    tear_down(conn, &table_name).await
}
