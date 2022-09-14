use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::prelude::Geometry;
use crate::types::{GeoType, ReqlType};
use crate::Command;

use super::point::Point;
use super::polygon_sub;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Geometry)]
pub struct Polygon {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: Vec<Vec<[f64; 2]>>,
    #[serde(rename = "type")]
    pub typ: GeoType,
}

impl Polygon {
    pub fn new(points: &[Point]) -> Self {
        assert!(points.len() >= 3);

        Self {
            reql_type: ReqlType::Geometry,
            typ: GeoType::Polygon,
            coordinates: vec![points.iter().map(|point| point.coordinates).collect()],
        }
    }

    pub fn new_from_vec(coordinates: Vec<Vec<[f64; 2]>>) -> Self {
        Self {
            reql_type: ReqlType::Geometry,
            typ: GeoType::Polygon,
            coordinates,
        }
    }

    pub fn polygon_sub(self, polygon: Polygon) -> Command {
        polygon_sub::new(polygon).with_parent(self.into())
    }
}

impl From<Polygon> for Command {
    fn from(polygon: Polygon) -> Self {
        polygon.coordinates.iter().flatten().fold(
            Command::new(TermType::Polygon),
            |command, coord| {
                let point: Command = Point::new(coord[0], coord[1]).into();

                command.with_arg(point)
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::{AnyParam, Point, Polygon};
    use crate::{r, Result};

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
        table
            .clone()
            .insert(AnyParam::new(&rectangle))
            .run(&conn)
            .await?;
        let response: Rectangle = table.get(1).run(&conn).await?.unwrap().parse()?;

        assert!(response == rectangle);

        tear_down(conn, &table_name).await
    }
}
