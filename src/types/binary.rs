use serde::{Deserialize, Serialize};

use super::ReqlType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Binary {
    #[serde(rename = "$reql_type$")]
    reql_type: ReqlType,
    pub data: String,
}

impl Binary {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            reql_type: ReqlType::Binary,
            data: base64::encode(bytes),
        }
    }
}
