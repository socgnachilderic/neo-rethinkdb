use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::ToGeojson)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::cmd::point::Point;
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{AnyParam, GeoJson, GeoType};
    use crate::{r, Result};

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
        let (conn, table) = set_up(TABLE_NAMES[0], false).await?;
        table
            .clone()
            .insert(AnyParam::new(&user))
            .run(&conn)
            .await?;
        let location: GeoJson<[f64; 2]> = table
            .get(1)
            .g("location")
            .to_geojson()
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(location == geo);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
