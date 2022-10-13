use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::types::{GeoType, ReqlType};
use crate::{Command, Geometry};

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

    /// Use `polygon2` to “punch out” a hole in `polygon1`.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// polygon1.polygon_sub(polygon2) → polygon
    /// ```
    ///
    /// Where:
    /// - polygon1, polygon1, polygon: [Polygon](crate::cmd::polygon::Polygon)
    ///
    /// # Description
    ///
    /// `polygon2` must be completely contained within `polygon1` and must
    /// have no holes itself (it must not be the output of `polygon_sub` itself).
    ///
    /// ## Examples
    ///
    /// Define a polygon with a hole punched in it.
    ///
    /// ```
    /// use neor::types::Polygon;
    /// use neor::{args, r, Converter, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let expected_data = Polygon::new_from_vec(vec![
    ///         vec![
    ///             [-122.4, 37.7],
    ///             [-122.4, 37.3],
    ///             [-121.8, 37.3],
    ///             [-121.8, 37.7],
    ///             [-122.4, 37.7],
    ///         ],
    ///         vec![
    ///             [-122.3, 37.4],
    ///             [-122.3, 37.6],
    ///             [-122.0, 37.6],
    ///             [-122.0, 37.4],
    ///             [-122.3, 37.4],
    ///         ],
    ///     ]);
    ///     let outer_polygon = r.polygon(&[
    ///         r.point(-122.4, 37.7),
    ///         r.point(-122.4, 37.3),
    ///         r.point(-121.8, 37.3),
    ///         r.point(-121.8, 37.7),
    ///     ]);
    ///     let inner_polygon = r.polygon(&[
    ///         r.point(-122.3, 37.4),
    ///         r.point(-122.3, 37.6),
    ///         r.point(-122.0, 37.6),
    ///         r.point(-122.0, 37.4),
    ///     ]);
    ///
    ///     let response: Polygon = outer_polygon
    ///         .polygon_sub(inner_polygon)
    ///         .run(&conn)
    ///         .await?
    ///         .unwrap()
    ///         .parse()?;
    ///
    ///     assert!(response == expected_data);
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [polygon](crate::r::polygon)
    pub fn polygon_sub(self, polygon: Polygon) -> Command {
        polygon_sub::new(polygon).with_parent(&self.into())
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
