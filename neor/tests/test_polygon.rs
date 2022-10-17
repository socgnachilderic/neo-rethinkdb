use neor::types::{Point, Polygon};
use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Rectangle {
    id: u8,
    rectangle: Polygon,
}

#[tokio::test]
async fn test_polygon_data() -> Result<()> {
    let rectangle = Rectangle {
        id: 1,
        rectangle: r.polygon(&[
            Point::new(-122.423246, 37.779388),
            Point::new(-122.423246, 37.329898),
            Point::new(-121.886420, 37.329898),
            Point::new(-121.886420, 37.779388),
        ]),
    };
    let (conn, table, table_name) = set_up(false).await?;
    table.insert(&rectangle).run(&conn).await?;
    let response: Rectangle = table.get(1).run(&conn).await?.unwrap().parse()?;

    assert!(response == rectangle);

    tear_down(conn, &table_name).await
}
