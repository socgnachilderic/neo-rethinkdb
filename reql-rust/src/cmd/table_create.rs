use super::{StaticString, run};
use crate::Command;
use crate::types::{DbResponseType, Durability, Replicas};
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;

pub struct TableCreateBuilder(Command, TableCreateOption);

#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub struct TableCreateOption {
    pub primary_key: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
}

impl TableCreateBuilder {
    pub fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::TableCreate).with_arg(args);

        Self(command, TableCreateOption::default())
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DbResponseType>> {       
        let cmd = self.0.with_opts(self.1).into_arg::<()>()
            .into_cmd();

        cmd.run::<_, DbResponseType>(arg).try_next().await
    }

    /// The name of the primary key. The default primary key is id.
    pub fn with_primary_key(mut self, primary_key_name: &'static str) -> Self {
        self.1.primary_key = Some(primary_key_name.static_string());
        self
    }

    /// If set to `soft`, writes will be acknowledged by the server immediately and flushed to disk in 
    /// the background. The default is `hard`: acknowledgment of writes happens after data has been 
    pub fn with_durability(mut self, durability: Durability) -> Self {
        self.1.durability = Some(durability);
        self
    }

    /// The number of shards, an integer from 1-64. Defaults to 1.
    pub fn with_shards(mut self, shards: u8) -> Self {
        assert!(shards >= 1 && shards <= 64);

        self.1.shards = Some(shards);
        self
    }

    /// Either an integer or a mapping object. Defaults to `Replicas::Int(1)`.
    pub fn with_replicas(mut self, replicas: Replicas) -> Self {
        self.1.replicas = Some(replicas);
        self
    }

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
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
            Int(u8),
            Map(&'a HashMap<Cow<'static, str>, u8>),
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
