use neor::types::{GeoJson, GeoType, ReqlGeoJson};
use neor::{r, Converter, Result};
use serde::{Deserialize, Serialize};

use common::{set_up, tear_down};

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: u8,
    name: String,
    location: ReqlGeoJson<[f64; 2]>,
}

#[tokio::test]
async fn test_geojson_data() -> Result<()> {
    let geo_json = GeoJson {
        typ: GeoType::Point,
        coordinates: [-122.423246, 37.779388],
    };
    let user = User {
        id: 1,
        name: "Yaoundé".to_string(),
        location: r.geojson(geo_json),
    };
    let (conn, table, table_name) = set_up(false).await?;
    table.insert(&user).run(&conn).await?;
    let response: User = table.get(1).run(&conn).await?.unwrap().parse()?;

    assert!(response == user);

    tear_down(conn, &table_name).await
}
