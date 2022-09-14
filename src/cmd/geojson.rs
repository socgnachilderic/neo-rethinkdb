use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::prelude::Geometry;
use crate::types::{GeoJson, GeoType, ReqlType};
use crate::Command;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ReqlGeoJson<T> {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: T,
    #[serde(rename = "type")]
    pub typ: GeoType,
}

impl<T: Serialize> ReqlGeoJson<T> {
    pub fn new(geojson: GeoJson<T>) -> Self {
        Self {
            reql_type: ReqlType::Geometry,
            typ: geojson.typ,
            coordinates: geojson.coordinates,
        }
    }
}

impl<T: Serialize> From<ReqlGeoJson<T>> for Command {
    fn from(geo: ReqlGeoJson<T>) -> Self {
        let geo: GeoJson<T> = geo.into();
        let arg = Command::from_json(geo);

        Command::new(TermType::Geojson).with_arg(arg)
    }
}

impl<T: Serialize> From<ReqlGeoJson<T>> for GeoJson<T> {
    fn from(geo: ReqlGeoJson<T>) -> Self {
        Self {
            typ: geo.typ,
            coordinates: geo.coordinates,
        }
    }
}

impl<T: Serialize> Geometry for ReqlGeoJson<T> {
    fn get_command(self) -> Command {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{AnyParam, GeoJson, GeoType};
    use crate::{r, Result};

    use super::ReqlGeoJson;

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
        let (conn, table) = set_up(TABLE_NAMES[0], false).await?;
        table
            .clone()
            .insert(AnyParam::new(&user))
            .run(&conn)
            .await?;
        let response: User = table.get(1).run(&conn).await?.unwrap().parse()?;

        assert!(response == user);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
