use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, Deserialize};

use crate::Command;
use crate::constants::{MAX_LONGITUDE_VALUE, MAX_LATITUDE_VALUE};
use crate::types::{QueryTypeResponse, ReqlType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: [f64; 2],
    #[serde(rename = "type")]
    pub typ: QueryTypeResponse,

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
            typ: QueryTypeResponse::Point,
            command: Some(command),
        }
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Self>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Self>> {
        self.command.unwrap().into_arg::<()>().into_cmd().run::<_, Self>(arg)
    }
}
