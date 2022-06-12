use serde::{Serialize, Deserialize};

use crate::constants::{MAX_LONGITUDE_VALUE, MAX_LATITUDE_VALUE};

use super::{ReqlType, QueryTypeResponse};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Point {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub coordinates: [f64; 2],
    #[serde(rename = "type")]
    pub typ: QueryTypeResponse,
}

impl Point {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        assert!(longitude <= MAX_LONGITUDE_VALUE && longitude >= -MAX_LONGITUDE_VALUE);
        assert!(latitude <= MAX_LATITUDE_VALUE && latitude >= -MAX_LATITUDE_VALUE);

        Self {
            reql_type: ReqlType::Geometry,
            coordinates: [longitude, latitude],
            typ: QueryTypeResponse::Point,
        }
    }
}
