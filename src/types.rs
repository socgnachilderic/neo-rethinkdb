use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

pub use crate::cmd::line::Line;
pub use crate::cmd::point::Point;
pub use crate::cmd::polygon::Polygon;
pub use binary::Binary;
pub use datetime::DateTime;
pub use group_stream::{GroupItem, GroupStream};
pub use time_::Time;

pub(crate) use datetime::timezone_to_string;

use crate::Command;

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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
    pub config_changes: Vec<ConfigChange<ConfigChangeValue>>,
    pub dbs_created: Option<usize>,
    pub dbs_dropped: Option<usize>,
    pub tables_created: Option<usize>,
    pub tables_dropped: Option<usize>,
}

/// Structure of return data in `db` table
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct WritingResponse<T> {
    pub inserted: usize,
    pub replaced: usize,
    pub unchanged: usize,
    pub skipped: usize,
    pub deleted: usize,
    pub errors: usize,
    pub first_error: Option<String>,
    pub generated_keys: Option<Vec<Uuid>>,
    pub warnings: Option<Vec<String>>,
    pub changes: Option<Vec<ConfigChange<T>>>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitResponse {
    pub ready: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinResponse<L, R> {
    pub left: Option<L>,
    pub right: Option<R>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct GrantResponse {
    pub granted: u8,
    pub permissions_changes: Vec<ConfigChange<GrantChangeValue>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RebalanceResponse {
    pub rebalanced: u8,
    pub status_changes: Vec<ConfigChange<StatusResponse>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct ReconfigureResponse {
    pub reconfigured: u8,
    pub config_changes: Vec<ConfigChange<ConfigChangeValue>>,
    pub status_changes: Vec<ConfigChange<StatusResponse>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
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
pub struct StatusResponse {
    pub db: Option<Cow<'static, str>>,
    pub id: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub raft_leader: Option<Cow<'static, str>>,
    pub shards: Option<Vec<ShardType<ShardReplicasType>>>,
    pub status: Option<StatusResponseStatus>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Replicas {
    Int(u8),
    Map {
        replicas: HashMap<Cow<'static, str>, u8>,
        primary_replica_tag: Cow<'static, str>,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, PartialOrd)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
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
#[serde(rename_all = "snake_case")]
pub enum EmergencyRepair {
    UnsafeRollback,
    UnsafeRollbackOrErase,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Debug, Clone)]
pub struct AnyParam(Command);

impl AnyParam {
    pub fn new(arg: impl Serialize) -> Self {
        Self(Command::from_json(arg))
    }
}

impl From<AnyParam> for Command {
    fn from(param: AnyParam) -> Self {
        param.0
    }
}
