use neor::types::{GeoJson, GeoType, Point};
use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: u8,
    name: String,
    location: Point,
}

#[tokio::test]
async fn test_to_geojson_ops() -> Result<()> {
    let user = User {
        id: 1,
        name: "sfo".to_string(),
        location: r.point(-122.423246, 37.779388),
    };
    let geo: GeoJson<[f64; 2]> = GeoJson {
        typ: GeoType::Point,
        coordinates: [-122.423246, 37.779388],
    };
    let (conn, table, table_name) = set_up(false).await?;
    table.insert(&user).run(&conn).await?;
    let location: GeoJson<[f64; 2]> = table
        .get(1)
        .g("location")
        .to_geojson()
        .run(&conn)
        .await?
        .unwrap()
        .parse()?;

    assert!(location == geo);

    tear_down(conn, &table_name).await
}
