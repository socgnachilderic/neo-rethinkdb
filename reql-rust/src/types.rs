
use std::{borrow::Cow, collections::HashMap};

#[doc(inline)]
pub use reql_rust_types::*;
use serde::{Deserialize, Serialize, Serializer};

/// Structure of return data in `db_create` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbCreateReturnType {
    pub config_changes: Vec<ConfigChange>,
    pub dbs_created: u32,
}

/// Structure of return data in `db_drop` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbDropReturnType {
    pub config_changes: Vec<ConfigChange>,
    pub tables_dropped: u32,
    pub dbs_dropped: u32,
}

/// Structure of return data in `table_create` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct TableCreateReturnType {
    pub config_changes: Vec<ConfigChange>,
    pub tables_created: u32,
}

/// Structure of return data in `table_drop` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct TableDropReturnType {
    pub config_changes: Vec<ConfigChange>,
    pub tables_dropped: u32,
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
