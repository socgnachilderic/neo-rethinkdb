use std::{borrow::Cow, collections::HashMap};

use neor_macros::CommandOptions;
use serde::{Deserialize, Serialize};

pub use any_param::AnyParam;
pub use options::*;
pub use return_changes::ReturnChanges;

mod any_param;
mod options;
mod return_changes;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Args<T>(pub T);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct Permission {
    /// allows reading the data in tables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// allows modifying data, including inserting, replacing/updating, and deleting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    /// allows a user to open HTTP connections via the [http](crate::r::http)
    /// command. This permission can only be granted in global scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
    /// allows users to create/drop
    /// [secondary indexes](https://rethinkdb.com/docs/secondary-indexes/python/)
    /// on a table and changing the cluster configuration;
    /// to create and drop tables, if granted on a database;
    /// and to create and drop databases, if granted globally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Open,
    Closed,
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
    /// returns values that are in memory
    /// (but not necessarily written to disk)
    /// on the primary replica.
    /// This is the default.
    Single,
    /// will only return values that are safely committed on disk on a majority of replicas.
    /// This requires sending a message to every replica on each read,
    /// so it is the slowest but most consistent.
    Majority,
    /// will return values that are in memory on an arbitrarily-selected replica.
    /// This is the fastest but least consistent.
    Outdated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Native,
    Raw,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Conflict {
    Error,
    Replace,
    Update,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Replicas {
    Int(usize),
    Map {
        replicas: HashMap<Cow<'static, str>, usize>,
        primary_replica_tag: Cow<'static, str>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EmergencyRepair {
    UnsafeRollback,
    UnsafeRollbackOrErase,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WaitFor {
    ReadyForOutdatedReads,
    ReadyForReads,
    ReadyForWrites,
    AllReplicasReady,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Interleave {
    Bool(bool),
    FieldName(&'static str),
    // Function
}

/// Controls how change notifications are batched
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Squash {
    Bool(bool),
    Float(f32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeoSystem {
    #[serde(rename = "unit_sphere")]
    UnitSphere,
    WGS84,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoerceType {
    Array,
    String,
    Number,
    Object,
    Binary,
}
