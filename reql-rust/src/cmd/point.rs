use std::fmt::Debug;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::constants::{MAX_LATITUDE_VALUE, MAX_LONGITUDE_VALUE};
use crate::ops::{ReqlOps, ReqlOpsGeometry};
use crate::types::{GeoType, ReqlType};
use crate::Command;

#[derive(Serialize, Deserialize, Clone)]
pub struct Point {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: [f64; 2],
    #[serde(rename = "type")]
    pub typ: GeoType,

    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) command: Option<Command>,
}

impl Point {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        assert!(longitude <= MAX_LONGITUDE_VALUE && longitude >= -MAX_LONGITUDE_VALUE);
        assert!(latitude <= MAX_LATITUDE_VALUE && latitude >= -MAX_LATITUDE_VALUE);
        let mut command = Command::new(TermType::Point);

        for coord in [longitude, latitude] {
            let arg = Command::from_json(coord);
            command = command.with_arg(arg);
        }

        Self {
            reql_type: ReqlType::Geometry,
            coordinates: [longitude, latitude],
            typ: GeoType::Point,
            command: Some(command),
        }
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Self>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Self>> {
        self.get_parent().run::<_, Self>(arg)
    }
}

impl ReqlOpsGeometry for Point {}

impl ReqlOps for Point {
    fn get_parent(&self) -> Command {
        self.command.clone().unwrap().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for Point {
    fn into(self) -> Command {
        self.get_parent()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("reql_type", &self.reql_type)
            .field("coordinates", &self.coordinates)
            .field("typ", &self.typ)
            .finish()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coordinates.partial_cmp(&other.coordinates)
    }
}
