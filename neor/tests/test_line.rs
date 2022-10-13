use neor::types::{Line, Point};
use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Route {
    id: u8,
    route: Line,
}

#[tokio::test]
async fn test_line_data() -> Result<()> {
    let route = Route {
        id: 1,
        route: r.line(&[
            Point::new(-122.423246, 37.779388),
            Point::new(-121.886420, 37.329898),
        ]),
    };
    let (conn, table, table_name) = set_up(false).await?;
    table.insert(&route).run(&conn).await?;
    let response: Route = table.get(1).run(&conn).await?.unwrap().parse()?;

    assert!(response == route);

    tear_down(conn, &table_name).await
}
