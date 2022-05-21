
use std::{borrow::Cow, collections::HashMap};

#[doc(inline)]
pub use reql_rust_types::*;
use serde::{Deserialize, Serialize, Serializer};

/// Structure of return data in `db` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbResponseType {
    pub config_changes: Vec<ConfigChange>,
    pub dbs_created: Option<u32>,
    pub dbs_dropped: Option<u32>,
    pub tables_created: Option<u32>,
    pub tables_dropped: Option<u32>,
}

/// Structure of return data in `index` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct IndexResponseType {
    pub created: Option<u32>,
    pub dropped: Option<u32>,
    pub renamed: Option<u8>,
}

/// Structure of return data in `index_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct IndexStatusResponseType {
    pub geo: bool,
    pub ready: bool,
    pub multi: bool,
    pub outdated: bool,
    pub progress: Option<f64>,
    pub index: Cow<'static, str>,
    pub function: Binary,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ConfigChange {
    pub new_val: Option<ConfigChangeValue>,
    pub old_val: Option<ConfigChangeValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ConfigChangeValue {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,

    pub db: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub indexes: Option<Vec<Cow<'static, str>>>,
    pub primary_key: Option<Cow<'static, str>>,
    pub shards: Option<Vec<ShardType>>,
    pub write_acks: Option<ReadMode>,
    pub write_hook: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ShardType {
    pub primary_replica: Cow<'static, str>,
    pub replicas: Vec<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Replicas {
    Int(u8),
    Map {
        replicas: HashMap<Cow<'static, str>, u8>,
        primary_replica_tag: Cow<'static, str>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReturnChanges {
    Bool(bool),
    Always,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

impl Serialize for ReturnChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Always => "always".serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}
