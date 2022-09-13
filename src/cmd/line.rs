use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::types::{GeoType, ReqlType};
use crate::Command;
use crate::prelude::Geometry;

use super::point::Point;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Geometry)]
pub struct Line {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: Vec<[f64; 2]>,
    #[serde(rename = "type")]
    pub typ: GeoType,
}

impl Line {
    pub fn new(points: &[Point]) -> Self {
        assert!(points.len() >= 2);

        Self {
            reql_type: ReqlType::Geometry,
            typ: GeoType::LineString,
            coordinates: points.iter().map(|point| point.coordinates).collect(),
        }
    }

    pub fn fill(self) -> Command {
        super::fill::new().with_parent(self.into())
    }
}

impl Into<Command> for Line {
    fn into(self) -> Command {
        self.coordinates
            .iter()
            .fold(Command::new(TermType::Line), |command, coord| {
                let point: Command = Point::new(coord[0], coord[1]).into();

                command.with_arg(point)
            })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{AnyParam, Line, Point};
    use crate::{r, Result};

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
        let (conn, table) = set_up(TABLE_NAMES[0], false).await?;
        table
            .clone()
            .insert(AnyParam::new(&route))
            .run(&conn)
            .await?;
        let response: Route = table.get(1).run(&conn).await?.unwrap().parse()?;

        assert!(response == route);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
