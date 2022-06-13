use std::fmt::Debug;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::Command;
use crate::ops::{ReqlOps, ReqlOpsGeometry};
use crate::types::{QueryTypeResponse, ReqlType};

use super::point::Point;

#[derive(Serialize, Deserialize, Clone)]
pub struct Line {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: Vec<[f64; 2]>,
    #[serde(rename = "type")]
    pub typ: QueryTypeResponse,

    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) command: Option<Command>,
}

impl Line {
    pub fn new(points: &[Point]) -> Self {
        assert!(points.len() >= 2);
        let mut command = Command::new(TermType::Line);
        let mut coordinates: Vec<[f64; 2]> = Vec::new();

        for point in points.iter() {
            command = command.with_arg(point.command.clone().unwrap());
            coordinates.push(point.coordinates);
        }

        Self {
            coordinates,
            command: Some(command),
            reql_type: ReqlType::Geometry,
            typ: QueryTypeResponse::LineString,
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

    /// Convert a Line object into a Polygon object. 
    /// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them.
    /// 
    /// ## Example
    /// 
    /// Create a line object and then convert it to a polygon.
    /// 
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// 
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let rectangle = [
    ///         r.point(-122.423246,37.779388),
    ///         r.point(-122.423246,37.329898),
    ///         r.point(-121.886420,37.329898),
    ///         r.point(-121.886420,37.779388),
    ///     ];
    /// 
    ///     let _ = r.line(&rectangle)
    ///         .fill()
    ///         .run(&session)
    ///         .await?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn fill(&self) -> super::fill::FillBuilder {
        super::fill::FillBuilder::new()._with_parent(self.get_parent())
    }
}

impl ReqlOpsGeometry for Line { }

impl ReqlOps for Line {
    fn get_parent(&self) -> Command {
        self.command.clone().unwrap()
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line")
            .field("reql_type", &self.reql_type)
            .field("coordinates", &self.coordinates)
            .field("typ", &self.typ)
            .finish()
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
