use std::fmt::Debug;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::ops::{ReqlOps, ReqlOpsGeometry};
use crate::types::{GeoType, ReqlType};
use crate::Command;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqlGeoJson<T> {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: T,
    #[serde(rename = "type")]
    pub typ: GeoType,

    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) command: Option<Command>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeoJson<T: Serialize + Copy> {
    #[serde(rename = "type")]
    pub typ: GeoType,
    pub coordinates: T,
}

impl<T: Unpin + Serialize + DeserializeOwned + Copy> ReqlGeoJson<T> {
    pub fn new(geojson: &GeoJson<T>) -> Self {
        let arg = Command::from_json(geojson.clone());
        let command = Command::new(TermType::Geojson).with_arg(arg);

        Self {
            command: Some(command),
            reql_type: ReqlType::Geometry,
            typ: geojson.typ,
            coordinates: geojson.coordinates,
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
}

impl<T: Serialize + Copy> GeoJson<T> {
    pub fn new(typ: GeoType, coordinates: T) -> Self {
        Self { typ, coordinates }
    }
}

impl<T> ReqlOpsGeometry for ReqlGeoJson<T> {}

impl<T> ReqlOps for ReqlGeoJson<T> {
    fn get_parent(&self) -> Command {
        self.command.clone().unwrap()
    }
}

impl<T: Debug> Debug for ReqlGeoJson<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqlGeoJson")
            .field("reql_type", &self.reql_type)
            .field("coordinates", &self.coordinates)
            .field("typ", &self.typ)
            .finish()
    }
}
