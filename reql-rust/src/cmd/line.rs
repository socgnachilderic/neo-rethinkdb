use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, Deserialize};

use crate::Command;
use crate::types::{QueryTypeResponse, ReqlType};

use super::point::Point;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Line {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: Vec<[f64; 2]>,
    #[serde(rename = "type")]
    pub typ: QueryTypeResponse,

    #[serde(skip_deserializing, skip_serializing)]
    command: Option<Command>,
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
        self.command.unwrap().into_arg::<()>().into_cmd().run::<_, Self>(arg)
    }
}
