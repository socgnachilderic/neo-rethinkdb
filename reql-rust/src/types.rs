use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

// pub use crate::cmd::line::Line;
// pub use crate::cmd::point::Point;
// pub use crate::cmd::polygon::Polygon;
// pub use document::Document;
// pub use group_stream::{GroupItem, GroupStream};
// pub use sequence::Sequence;
// pub use datetime::DateTime;
pub use binary::Binary;

// mod document;
// mod group_stream;
// mod sequence;
// mod datetime;
mod binary;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "UPPERCASE")]
pub enum ReqlType {
    Geometry,
    GroupStream,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum GeoType {
    LineString,
    Point,
    Polygon,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ServerInfo {
    pub id: Uuid,
    pub proxy: bool,
    pub name: Option<String>,
}

/// Structure of return data in `db` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct DbResponse {
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
    pub inserted: u32,
    pub replaced: u32,
    pub unchanged: u32,
    pub skipped: u32,
    pub deleted: u32,
    pub errors: u32,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<Uuid>>,
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Vec<ConfigChange<T>>>,
}

/// Structure of return data in `index` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct IndexResponse {
    pub created: Option<u32>,
    pub dropped: Option<u32>,
    pub renamed: Option<u8>,
}

/// Structure of return data in `index_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct IndexStatusResponse {
    pub geo: bool,
    pub ready: bool,
    pub multi: bool,
    pub outdated: bool,
    pub progress: Option<f64>,
    pub index: Cow<'static, str>,
    pub query: Cow<'static, str>,
    pub function: Binary,
}

/// Structure of return data in `index_status` table
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WriteHookResponse {
    pub function: Binary,
    pub query: Cow<'static, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UngroupResponseType<G, V> {
    pub group: G,
    pub reduction: Vec<V>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncResponseType {
    synced: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitResponseType {
    ready: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinResponseType<L, R> {
    pub left: Option<L>,
    pub right: Option<R>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantResponseType {
    pub granted: u8,
    pub permissions_changes: Vec<ConfigChange<GrantChangeValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalanceResponseType {
    pub rebalanced: u8,
    pub status_changes: Vec<ConfigChange<StatusResponseType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigureResponseType {
    pub reconfigured: u8,
    pub config_changes: Vec<ConfigChange<ConfigChangeValue>>,
    pub status_changes: Vec<ConfigChange<StatusResponseType>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigChange<T> {
    pub new_val: Option<T>,
    pub old_val: Option<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigChangeValue {
    pub id: Cow<'static, str>,
    pub name: Cow<'static, str>,

    pub db: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub indexes: Option<Vec<Cow<'static, str>>>,
    pub primary_key: Option<Cow<'static, str>>,
    pub shards: Option<Vec<ShardType<Cow<'static, str>>>>,
    pub write_acks: Option<ReadMode>,
    pub write_hook: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GrantChangeValue {
    pub write: Option<bool>,
    pub read: Option<bool>,
    pub connect: Option<bool>,
    pub config: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StatusResponseType {
    pub db: Option<Cow<'static, str>>,
    pub id: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub raft_leader: Option<Cow<'static, str>>,
    pub shards: Option<Vec<ShardType<ShardReplicasType>>>,
    pub status: Option<StatusResponseTypeStatus>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StatusResponseTypeStatus {
    pub all_replicas_ready: Option<bool>,
    pub ready_for_outdated_reads: Option<bool>,
    pub ready_for_reads: Option<bool>,
    pub ready_for_writes: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShardType<R> {
    pub primary_replica: Option<Cow<'static, str>>,
    pub replicas: Vec<R>,
    pub nonvoting_replicas: Option<Vec<Cow<'static, str>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShardReplicasType {
    pub server: Cow<'static, str>,
    pub state: Cow<'static, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeoJson<T: Serialize + Clone> {
    #[serde(rename = "type")]
    pub typ: GeoType,
    pub coordinates: T,
}

impl<T: Serialize + Clone> GeoJson<T> {
    pub fn new(typ: GeoType, coordinates: T) -> Self {
        Self { typ, coordinates }
    }
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WaitFor {
    ReadyForOutdatedReads,
    ReadyForReads,
    ReadyForWrites,
    AllReplicasReady,
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

#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Interleave {
    Bool(bool),
    FieldName(&'static str),
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EmergencyRepair {
    UnsafeRollback,
    UnsafeRollbackOrErase,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Unit {
    #[serde(rename = "m")]
    Meter,
    #[serde(rename = "km")]
    Kilometer,
    #[serde(rename = "mi")]
    InternationalMile,
    #[serde(rename = "nm")]
    NauticalMile,
    #[serde(rename = "ft")]
    InternationalFoot,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeoSystem {
    #[serde(rename = "unit_sphere")]
    UnitSphere,
    WGS84,
}
