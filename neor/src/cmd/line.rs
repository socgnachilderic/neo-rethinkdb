use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::types::{GeoType, ReqlType};
use crate::{Command, Geometry};

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

    /// Convert a Line object into a Polygon object.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// line.fill() → polygon
    /// ```
    ///
    /// Where:
    /// - polygon: [Polygon](crate::cmd::polygon::Polygon)
    ///
    /// # Description
    ///
    /// If the last point does not specify the same coordinates as the first point,
    /// `polygon` will close the polygon by connecting them.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90)
    /// of vertices are plotted on a perfect sphere.
    /// See [Geospatial support](https://rethinkdb.com/docs/geo-support/python/)
    /// for more information on ReQL’s coordinate system.
    ///
    /// If the last point does not specify the same coordinates as the first point,
    /// `polygon` will close the polygon by connecting them.
    /// You cannot directly construct a polygon with holes in it using `polygon`,
    /// but you can use [polygon_sub](crate::types::Polygon::polygon_sub)
    /// to use a second polygon within the interior of the first to define a hole.
    ///
    /// ## Examples
    ///
    /// Create a line object and then convert it to a polygon.
    ///
    /// ```
    /// use neor::{args, r, Result};
    /// use serde_json::json;
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///
    ///     let response = r.line(&[
    ///         r.point(-122.423246, 37.779388),
    ///         r.point(-122.423246, 37.329898),
    ///         r.point(-121.886420, 37.329898),
    ///         r.point(-121.886420, 37.779388),
    ///     ])
    ///     .fill()
    ///     .run(&conn)
    ///     .await?;
    ///
    ///     assert!(response.is_some());
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [polygon](crate::r::polygon)
    /// - [line](crate::r::line)
    pub fn fill(self) -> Command {
        super::fill::new().with_parent(&self.into())
    }
}

impl From<Line> for Command {
    fn from(line: Line) -> Self {
        line.coordinates
            .iter()
            .fold(Command::new(TermType::Line), |command, coord| {
                let point: Command = Point::new(coord[0], coord[1]).into();

                command.with_arg(point)
            })
    }
}
