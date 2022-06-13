use std::fmt::Debug;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::Command;
use crate::ops::{ReqlOpsGeometry, ReqlOps};
use crate::types::{GeoType, ReqlType};

use super::point::Point;
use super::polygon_sub::PolygonSubBuilder;

#[derive(Serialize, Deserialize, Clone)]
pub struct Polygon {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: Vec<Vec<[f64; 2]>>,
    #[serde(rename = "type")]
    pub typ: GeoType,

    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) command: Option<Command>,
}

impl Polygon {
    pub fn new(points: &[Point]) -> Self {
        assert!(points.len() >= 3);
        let mut command = Command::new(TermType::Polygon);
        let mut coordinates: Vec<[f64; 2]> = Vec::new();

        for point in points.iter() {
            command = command.with_arg(point.command.clone().unwrap());
            coordinates.push(point.coordinates);
        }

        Self {
            coordinates: vec![coordinates],
            command: Some(command),
            reql_type: ReqlType::Geometry,
            typ: GeoType::Polygon,
        }
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Self>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Self>> {
        self.command
            .unwrap()
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Self>(arg)
    }

    /// Use polygon2 to “punch out” a hole in polygon1. 
    /// polygon2 must be completely contained within polygon1 and must have no holes itself (it must not be the output of polygon_sub itself).
    /// 
    /// ## Example
    /// 
    /// Define a polygon with a hole punched in it.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use reql_rust::types::Point;
    /// use serde_json::{Value, json};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let outer_polygon = [
    ///        Point::new(-122.4, 37.7),
    ///        Point::new(-122.4, 37.3),
    ///        Point::new(-121.8, 37.3),
    ///        Point::new(-121.8, 37.7),
    ///    ];
    ///
    ///    let inner_polygon = [
    ///        Point::new(-122.3, 37.4),
    ///        Point::new(-122.3, 37.6),
    ///        Point::new(-122.0, 37.6),
    ///        Point::new(-122.0, 37.4),
    ///    ];
    ///
    ///    let outer_polygon = r.polygon(&outer_polygon);
    ///    let inner_polygon = r.polygon(&inner_polygon);
    ///
    ///    let _ = outer_polygon.polygon_sub(&inner_polygon).run(&session).await?;
    /// 
    ///    Ok(())
    /// }
    /// ```
    pub fn polygon_sub(&self, polygon: &Polygon) -> PolygonSubBuilder {
        PolygonSubBuilder::new(polygon)._with_parent(self.command.clone().unwrap())
    }
}

impl ReqlOpsGeometry for Polygon { }

impl ReqlOps for Polygon {
    fn get_parent(&self) -> Command {
        self.command.clone().unwrap()
    }
}

impl Debug for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Polygon")
            .field("reql_type", &self.reql_type)
            .field("coordinates", &self.coordinates)
            .field("typ", &self.typ)
            .finish()
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl PartialOrd for Polygon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
