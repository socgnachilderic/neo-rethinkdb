use ql2::term::TermType;
use serde::{Deserialize, Serialize};

use crate::types::{GeoJson, GeoType, ReqlType};
use crate::{Command, Geometry};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ReqlGeoJson<T> {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: T,
    #[serde(rename = "type")]
    pub typ: GeoType,
}

impl<T: Serialize> ReqlGeoJson<T> {
    pub fn new(geojson: GeoJson<T>) -> Self {
        Self {
            reql_type: ReqlType::Geometry,
            typ: geojson.typ,
            coordinates: geojson.coordinates,
        }
    }
}

impl<T: Serialize> From<ReqlGeoJson<T>> for Command {
    fn from(geo: ReqlGeoJson<T>) -> Self {
        let geo: GeoJson<T> = geo.into();
        let arg = Command::from_json(geo);

        Command::new(TermType::Geojson).with_arg(arg)
    }
}

impl<T: Serialize> From<ReqlGeoJson<T>> for GeoJson<T> {
    fn from(geo: ReqlGeoJson<T>) -> Self {
        Self {
            typ: geo.typ,
            coordinates: geo.coordinates,
        }
    }
}

impl<T: Serialize> Geometry for ReqlGeoJson<T> {
    fn cmd(self) -> Command {
        self.into()
    }
}
