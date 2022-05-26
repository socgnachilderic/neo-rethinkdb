
use std::{borrow::Cow, collections::HashMap};

#[doc(inline)]
pub use reql_rust_types::*;
use serde::{Deserialize, Serialize, Serializer};

/// Structure of return data in `db` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbResponseType {
    pub config_changes: Vec<ConfigChange<ConfigChangeValue>>,
    pub dbs_created: Option<u32>,
    pub dbs_dropped: Option<u32>,
    pub tables_created: Option<u32>,
    pub tables_dropped: Option<u32>,
}

/// Structure of return data in `db` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct WritingResponseType<T> {
    /// if return_changes is set to true, this will be an array of objects, one for each objected affected by the update operation. 
    /// Each object will have two keys: {new_val: <new value>, old_val: <old value>}.
    pub changes: Option<Vec<ConfigChange<T>>>,


    /// For an update operation.
    /// For an insert operation.
    pub deleted: Option<u32>,
    /// The number of errors encountered while performing the insert, update.
    pub errors: Option<u32>,
    /// If errors were encountered, contains the text of the first error.
    pub first_error: Option<u32>,
    /// A list of generated primary keys for inserted documents whose primary keys were not specified (capped to 100,000).
    pub generated_keys: Option<Vec<Cow<'static, str>>>,
    /// The number of documents successfully inserted.
    pub inserted: Option<u32>,

    /// The number of documents updated when `conflict` is set to `"replace"` or `"update"`.
    pub replaced: Option<u32>,
    /// The number of documents that were skipped because the document didn’t exist.
    /// For an insert operation.
    pub skipped: Option<u32>,
    /// The number of documents that would have been modified except the new value was the same as the old value.
    /// The number of documents updated when `conflict` is set to `"replace"` or `"update"`.
    pub unchanged: Option<u32>,
    /// If the field generated_keys is truncated, you will get the warning `“Too many generated keys (<X>), array truncated to 100000.”`.
    pub warnings: Option<u32>,
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

/// Structure of return data in `index_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WriteHookResponseType {
    pub function: Binary,
    pub query: Cow<'static, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncResponseType {
    synced: Option<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinResponseType<L, R> {
    left: Option<L>,
    right: Option<R>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ConfigChange<T> {
    pub new_val: Option<T>,
    pub old_val: Option<T>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Conflict {
    Error,
    Replace,
    Update,
}

/// Controls how change notifications are batched
#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Squash {
    /// `true`: When multiple changes to the same document occur before a
    /// batch of notifications is sent, the changes are "squashed" into one
    /// change. The client receives a notification that will bring it fully
    /// up to date with the server.
    /// `false`: All changes will be sent to the client verbatim. This is
    /// the default.
    Bool(bool),
    /// `n`: A numeric value (floating point). Similar to `true`, but the
    /// server will wait `n` seconds to respond in order to squash as many
    /// changes together as possible, reducing network traffic. The first
    /// batch will always be returned immediately.
    Float(f32),
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    Open,
    Closed,
}
