use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::constants::{MAX_LATITUDE_VALUE, MAX_LONGITUDE_VALUE};
use crate::prelude::Geometry;
use crate::types::{GeoType, ReqlType};
use crate::Command;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Geometry)]
pub struct Point {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: [f64; 2],
    #[serde(rename = "type")]
    pub typ: GeoType,
}

impl Point {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        assert!((-MAX_LONGITUDE_VALUE..=MAX_LONGITUDE_VALUE).contains(&longitude));
        assert!((-MAX_LATITUDE_VALUE..=MAX_LATITUDE_VALUE).contains(&latitude));

        Self {
            reql_type: ReqlType::Geometry,
            coordinates: [longitude, latitude],
            typ: GeoType::Point,
        }
    }
}

impl From<Point> for Command {
    fn from(point: Point) -> Self {
        point
            .coordinates
            .iter()
            .fold(Command::new(TermType::Point), |command, coord| {
                command.with_arg(Command::from_json(coord))
            })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{AnyParam, Point};
    use crate::{r, Result};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct User {
        id: u8,
        name: String,
        location: Point,
    }

    #[tokio::test]
    async fn test_point_data() -> Result<()> {
        let user = User {
            id: 1,
            name: "Yaoundé".to_string(),
            location: r.point(-122.423246, 37.779388),
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
