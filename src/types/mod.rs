use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::arguments::{Durability, ReadMode};
pub use crate::cmd::line::Line;
pub use crate::cmd::point::Point;
pub use crate::cmd::polygon::Polygon;
pub use binary::Binary;
pub use datetime::DateTime;
pub use group_stream::{GroupItem, GroupStream};
pub use time_::Time;

pub(crate) use datetime::timezone_to_string;

pub use crate::Command;

mod binary;
mod datetime;
mod group_stream;
mod time_;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[non_exhaustive]
#[serde(rename_all = "UPPERCASE")]
pub enum ReqlType {
    Geometry,
    GroupStream,
    Binary,
    Time,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum GeoType {
    LineString,
    Point,
    Polygon,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct ServerInfo {
    pub id: Uuid,
    pub proxy: bool,
    pub name: Option<String>,
}

/// Structure of return data in `db` table
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct DbResponse {
    pub config_changes: Vec<ConfigChange<ConfigResponse>>,
    pub dbs_created: Option<usize>,
    pub dbs_dropped: Option<usize>,
    pub tables_created: Option<usize>,
    pub tables_dropped: Option<usize>,
}

/// Structure of return data in `db` table
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
#[non_exhaustive]
pub struct WritingResponse {
    pub inserted: usize,
    pub replaced: usize,
    pub unchanged: usize,
    pub skipped: usize,
    pub deleted: usize,
    pub errors: usize,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<Uuid>>,
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Vec<ConfigChange<Value>>>,
}

/// Structure of return data in `index` table
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct IndexResponse {
    pub created: Option<usize>,
    pub dropped: Option<usize>,
    pub renamed: Option<usize>,
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
pub struct UngroupItem<G, V> {
    pub group: G,
    pub reduction: Vec<V>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncResponse {
    pub synced: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct WaitResponse {
    /// The value is an integer indicating the number of tables waited for.
    /// It will always be `1` when `wait` is called on a table,
    /// and the total number of tables when called on a database.
    pub ready: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinResponse<L, R> {
    pub left: Option<L>,
    pub right: Option<R>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct GrantResponse {
    /// The granted field will always be 1,
    pub granted: u8,
    /// list will have one object, describing the new permissions values and
    /// the old values they were changed from (which may be None).
    pub permissions_changes: Vec<ConfigChange<GrantChangeValue>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RebalanceResponse {
    /// the number of tables rebalanced.
    pub rebalanced: u8,
    /// a list of new and old table status values.
    /// Each element of the list will be an object with two fields:
    /// - `old_val`: The table’s [status](crate::Command::status)
    /// value before `rebalance` was executed.
    /// - `new_val`: The table’s `status` value after `rebalance` was executed.
    /// (This value will almost always indicate the table is unavailable.)
    pub status_changes: Vec<ConfigChange<StatusResponse>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ReconfigureResponse {
    /// the number of tables reconfigured.
    /// This will be `0` if `dry_run` is `true`.
    pub reconfigured: u8,
    /// a list of new and old table configuration values.
    /// Each element of the list will be an object with two fields
    /// - `old_val`: The table’s [config](crate::Command::config)
    /// value before reconfigure was executed.
    /// - `new_val`: The table’s `config` value after `reconfigure` was executed.
    pub config_changes: Vec<ConfigChange<ConfigResponse>>,
    /// a list of new and old table status values.
    /// Each element of the list will be an object with two fields
    /// - `old_val`: The table’s [status](crate::Command::status)
    /// value before reconfigure was executed.
    /// - `new_val`: The table’s `config` value after `reconfigure` was executed.
    pub status_changes: Vec<ConfigChange<StatusResponse>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct ConfigChange<T> {
    pub new_val: Option<T>,
    pub old_val: Option<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConfigResponse {
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
pub struct StatusResponse {
    /// the UUID of the table.
    pub id: Option<Cow<'static, str>>,
    /// the table’s name.
    pub name: Option<Cow<'static, str>>,
    /// the database the table is in.
    pub db: Option<Cow<'static, str>>,
    /// the subfields in this field indicate whether all shards of
    /// the table are ready to accept the given type of query
    pub status: Option<StatusResponseStatus>,
    /// one entry for each shard in `table_config`
    pub shards: Option<Vec<ShardType<ShardReplicasType>>>,
    pub raft_leader: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InfoResponse {
    pub db: DbInfo,
    pub doc_count_estimates: Vec<usize>,
    pub id: Cow<'static, str>,
    pub indexes: Vec<Cow<'static, str>>,
    pub name: Cow<'static, str>,
    pub primary_key: Cow<'static, str>,
    #[serde(rename = "type")]
    pub typ: TypeOf,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DbInfo {
    id: Cow<'static, str>,
    name: Cow<'static, str>,
    #[serde(rename = "type")]
    pub typ: TypeOf,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StatusResponseStatus {
    pub all_replicas_ready: Option<bool>,
    pub ready_for_outdated_reads: Option<bool>,
    pub ready_for_reads: Option<bool>,
    pub ready_for_writes: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct ClosestDocumentResponse<T> {
    pub dist: f64,
    pub doc: Option<T>,
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct GeoJson<T: Serialize> {
    #[serde(rename = "type")]
    pub typ: GeoType,
    pub coordinates: T,
}

impl<T: Serialize + Clone> GeoJson<T> {
    pub fn new(typ: GeoType, coordinates: T) -> Self {
        Self { typ, coordinates }
    }
}

/// Controls how change notifications are batched
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Squash {
    Bool(bool),
    Float(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Interleave {
    Bool(bool),
    FieldName(&'static str),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeoSystem {
    #[serde(rename = "unit_sphere")]
    UnitSphere,
    WGS84,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum TypeOf {
    Array,
    Bool,
    Db,
    Function,
    GroupedData,
    GroupedStream,
    Maxval,
    Minval,
    Null,
    Number,
    Object,

    #[serde(rename = "PTYPE<BINARY>")]
    PtypeBinary,
    #[serde(rename = "PTYPE<GEOMETRY>")]
    PtypeGeometry,
    #[serde(rename = "PTYPE<TIME>")]
    PtypeTime,
    #[serde(rename = "SELECTION<ARRAY>")]
    SelectionArray,
    #[serde(rename = "SELECTION<OBJECT>")]
    SelectionObject,
    #[serde(rename = "SELECTION<STREAM>")]
    SelectionStream,

    Stream,
    String,
    TableSlice,
    Table,
}
