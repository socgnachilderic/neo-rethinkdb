use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Byte {
    BINARY,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Binary {
    #[serde(rename = "$reql_type$")]
    reql_type: Byte,
    pub data: String,
}

impl Binary {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            reql_type: Byte::BINARY,
            data: base64::encode(bytes),
        }
    }
}
