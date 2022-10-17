use std::borrow::Cow;
use std::collections::HashMap;

use neor_macros::CommandOptions;
use serde::{Serialize, Serializer};

use crate::cmd::run::Db;
use crate::constants::DEFAULT_RETHINKDB_DBNAME;
use crate::Session;

use super::*;

#[derive(
    Debug, Clone, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, Hash, CommandOptions,
)]
pub struct CircleOption {
    /// the number of vertices in the polygon or line. Defaults to 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_vertices: Option<usize>,
    /// the reference ellipsoid to use for geographic coordinates.
    /// Possible values are `WGS84` (the default), a common standard
    /// for Earth’s geometry, or `UnitSphere`, a perfect sphere of 1 meter radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    /// Unit for the radius distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// if `true` (the default) the circle is filled, creating a polygon;
    /// if `false` the circle is unfilled (creating a line).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct ChangesOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<Squash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changefeed_queue_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_initial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_states: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_offsets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_types: Option<bool>,
}

///  If `index` is set to the name of a secondary index,
/// `between` will return all documents where that index’s
/// value is in the specified range (it uses the primary key by default).
/// `left_bound` or `right_bound` may be set to `Status::Open` or `Status::Closed`
/// to indicate whether or not to include that endpoint of the range
/// (by default, `left_bound` is closed and `right_bound` is open).
#[derive(
    Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
pub struct BetweenOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DeleteOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_atomic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, CommandOptions,
)]
pub struct DistanceOption {
    /// the reference ellipsoid to use for geographic coordinates.
    /// Possible values are `GeoSystem::WGS84` (the default),
    /// a common standard for Earth’s geometry, or `GeoSystem::UnitSphere`,
    /// a perfect sphere of 1 meter radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
    /// Unit to return the distance in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, CommandOptions)]
pub struct DistinctOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

/// By default, this is inclusive of the start time and exclusive of the end time.
/// Set left_bound and right_bound to explicitly include
/// (closed) or exclude (open) that endpoint of the range.
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DuringOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[derive(
    Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
#[non_exhaustive]
pub struct EqJoinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FilterOption {
    /// - If `default` is set to `true`, documents with missing
    /// fields will be returned rather than skipped.
    /// - If `default` is set to `r.error()`, an `ReqlRuntimeError` will
    /// be thrown when a document with a missing field is tested.
    /// - If `default` is set to `false` (the default),
    /// documents with missing fields will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl FilterOption {
    pub fn default_(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}

// #[derive(Debug, Clone, Serialize, Default)]
// #[non_exhaustive]
// pub struct FoldOption {
//     pub emit: Option<Command>,
//     pub final_emit: Option<Command>,
// }

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, CommandOptions)]
pub struct GetAllOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Serialize, Default, CommandOptions)]
pub struct GetIntersectingOption {
    pub index: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Serialize, Default, CommandOptions)]
pub struct GetNearestOption {
    pub index: Cow<'static, str>,
    /// the maximum number of results to return (default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<usize>,
    /// Unit for the distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// distance from an object to the specified point (default 100 km).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dist: Option<usize>,
    /// the reference ellipsoid to use for geographic coordinates.
    /// Possible values are `GeoSystem::WGS84` (the default),
    /// a common standard for Earth’s geometry, or `GeoSystem::UnitSphere`,
    /// a perfect sphere of 1 meter radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_system: Option<GeoSystem>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq, PartialOrd, Ord, CommandOptions)]
pub struct GroupOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct IndexCreateOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, CommandOptions)]
pub struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct InsertOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<Conflict>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub conflict_func: Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct JsOption {
    /// `timeout` is the number of seconds before r.js times out.
    /// The default value is 5 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct MaxOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct MinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct OrderByOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, CommandOptions,
)]
pub struct RandomOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, CommandOptions)]
#[non_exhaustive]
pub struct ReconfigureOption {
    /// the number of shards, an integer from 1-64. Required.
    pub shards: Option<u8>,
    /// either an usize or a mapping struct. Required.
    /// - If `replicas` is an usize, it specifies the number of replicas per shard.
    /// Specifying more replicas than there are servers will return an error.
    /// - If `replicas` is an struct, it specifies key-value pairs of server tags
    /// and the number of replicas to assign to those servers:
    /// `{"tag1": 2, "tag2": 4, "tag3": 2, ...}`.
    /// For more information about server tags, read
    /// [Administration tools](https://rethinkdb.com/docs/administration-tools/).
    pub replicas: Option<Replicas>,
    /// the generated configuration will not be applied to the table, only returned.
    pub dry_run: Option<bool>,
    /// Used for the Emergency Repair mode.
    /// See <https://rethinkdb.com/api/python/reconfigure#emergency-repair-mode>
    /// for more information.
    pub emergency_repair: Option<EmergencyRepair>,
}

impl Serialize for ReconfigureOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct InnerOptions<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            dry_run: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            emergency_repair: Option<EmergencyRepair>,
            #[serde(skip_serializing_if = "Option::is_none")]
            shards: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            replicas: Option<InnerReplicas<'a>>,
            /// the primary server specified by its server tag.
            /// Required if `replicas` is an object; the tag must be in the object.
            /// This must not be specified if `replicas` is an usize.
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_replica_tag: Option<&'a Cow<'static, str>>,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum InnerReplicas<'a> {
            Int(usize),
            Map(&'a HashMap<Cow<'static, str>, usize>),
        }

        let (replicas, primary_replica_tag) = match &self.replicas {
            Some(Replicas::Int(i)) => (Some(InnerReplicas::Int(*i)), None),
            Some(Replicas::Map {
                replicas,
                primary_replica_tag,
            }) => (
                Some(InnerReplicas::Map(replicas)),
                Some(primary_replica_tag),
            ),
            None => (None, None),
        };

        let opts = InnerOptions {
            dry_run: self.dry_run,
            emergency_repair: self.emergency_repair,
            replicas,
            primary_replica_tag,
            shards: self.shards,
        };

        opts.serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct ReplaceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_atomic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

#[derive(Debug, Clone, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
pub struct RunOption {
    /// One of three possible values affecting
    /// the consistency guarantee for the query (default: `ReadMode::Single`).
    /// - `ReadMode::Single` (the default) returns values that are in memory
    /// (but not necessarily written to disk) on the primary replica.
    /// - `ReadMode::Majority` will only return values that are safely
    /// committed on disk on a majority of replicas.
    /// This requires sending a message to every replica on each read,
    /// so it is the slowest but most consistent.
    /// - `ReadMode::Outdated` will return values that are in memory
    /// on an arbitrarily-selected replica.
    /// This is the fastest but least consistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    /// what format to return times in (default: `Format::Native`).
    /// Set this to `Format::Raw`
    /// if you want times returned as JSON objects for exporting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<Format>,
    /// whether or not to return a profile
    /// of the query’s execution (default: `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    /// possible values are `Durability::Hard` and `Durability::Soft`.
    /// In soft durability mode RethinkDB will acknowledge
    /// the write immediately after receiving it,
    /// but before the write has been committed to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    /// what format to return `grouped_data` and
    /// `grouped_streams` in (default: `Format::Native`).
    /// Set this to `Format::Raw` if you want the raw pseudotype.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noreply: Option<bool>,
    /// the database to run this query against as a string.
    /// The default is the database specified in
    /// the `db` [connection](crate::connection::Connection)
    /// method (which defaults to `test`).
    /// The database may also be specified with the db command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<Db>,
    /// the maximum numbers of array elements
    /// that can be returned by a query (default: 100,000).
    /// This affects all ReQL commands that return arrays.
    /// Note that it has no effect on the size of arrays
    /// being **written** to the database;
    /// those always have an upper limit of 100,000 elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_limit: Option<usize>,
    /// what format to return binary data in (default: `Format::Native`).
    /// Set this to `Format::Raw` if you want the raw pseudotype.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_format: Option<Format>,
    /// minimum number of rows to wait for before batching
    /// a result set (default: 8). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_batch_rows: Option<usize>,
    /// maximum number of rows to wait for before batching
    /// a result set (default: unlimited). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_rows: Option<usize>,
    /// maximum number of bytes to wait for before batching
    /// a result set (default: 1MB). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_bytes: Option<usize>,
    /// maximum number of seconds to wait before batching
    /// a result set (default: 0.5).
    /// This is a f64 and may be specified to the microsecond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_seconds: Option<f64>,
    /// factor to scale the other parameters down by on the first batch (default: 4).
    /// For example, with this set to 8 and `max_batch_rows` set to 80,
    /// on the first batch `max_batch_rows` will be adjusted to 10 (80 / 8).
    /// This allows the first batch to return faster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_batch_scaledown_factor: Option<usize>,
}

impl RunOption {
    pub async fn default_db(self, session: &Session) -> RunOption {
        let session_db = session.inner.db.lock().await;
        if self.db.is_none() && *session_db != DEFAULT_RETHINKDB_DBNAME {
            return self.db(&*session_db);
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct SliceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[derive(Debug, Default, Clone, PartialEq, CommandOptions)]
#[non_exhaustive]
pub struct TableCreateOption {
    pub primary_key: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
}

impl Serialize for TableCreateOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct InnerOptions<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_key: Option<&'a Cow<'static, str>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            durability: Option<Durability>,
            #[serde(skip_serializing_if = "Option::is_none")]
            shards: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            replicas: Option<InnerReplicas<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_replica_tag: Option<&'a Cow<'static, str>>,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum InnerReplicas<'a> {
            Int(usize),
            Map(&'a HashMap<Cow<'static, str>, usize>),
        }

        let (replicas, primary_replica_tag) = match &self.replicas {
            Some(Replicas::Int(i)) => (Some(InnerReplicas::Int(*i)), None),
            Some(Replicas::Map {
                replicas,
                primary_replica_tag,
            }) => (
                Some(InnerReplicas::Map(replicas)),
                Some(primary_replica_tag),
            ),
            None => (None, None),
        };

        let opts = InnerOptions {
            replicas,
            primary_replica_tag,
            primary_key: self.primary_key.as_ref(),
            durability: self.durability,
            shards: self.shards,
        };

        opts.serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
#[non_exhaustive]
pub struct TableOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_format: Option<IdentifierFormat>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct UnionOption {
    /// The optional `interleave` argument controls
    /// how the sequences will be merged:
    /// - `Interleave::Bool(true)`: results will be mixed together;
    /// this is the fastest setting, but ordering of elements is not guaranteed.
    /// (This is the default.)
    /// - `Interleave::Bool(false)`: input sequences will be appended to one another, left to right.
    /// - `Interleave::FieldName(field_name)`: a string will be taken as the name of a field
    /// to perform a merge-sort on. The input sequences must be ordered **before** being passed to `union`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interleave: Option<Interleave>,
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct UpdateOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_atomic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

#[derive(Debug, Copy, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct WaitOption {
    /// a enum indicating a table status to wait on before returning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<WaitFor>,
    /// a number indicating maximum time, in seconds,
    /// to wait for the table to be ready.
    /// If this value is exceeded, a ReqlRuntimeError will be thrown.
    /// A value of0 means no timeout. The default is 0 (no timeout).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}
