use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::constants::{MAX_LATITUDE_VALUE, MAX_LONGITUDE_VALUE};
use crate::types::{GeoType, ReqlType};
use crate::{Command, Geometry};

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
