use std::borrow::Cow;
use std::collections::HashMap;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, Serializer};

use crate::Command;
use crate::ops::ReqlOps;
use crate::types::{Replicas, EmergencyRepair, ReconfigureResponseType};

#[derive(Debug, Clone)]
pub struct ReconfigureBuilder(pub(crate) Command, pub(crate) ReconfigureOption);

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub(crate) struct ReconfigureOption {
    pub dry_run: Option<bool>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
    pub emergency_repair: Option<EmergencyRepair>,
}

impl ReconfigureBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Reconfigure);

        Self(command, ReconfigureOption::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<ReconfigureResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<ReconfigureResponseType>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, ReconfigureResponseType>(arg)
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.1.dry_run = Some(dry_run);
        self
    }

    pub fn with_shards(mut self, shards: u8) -> Self {
        self.1.shards = Some(shards);
        self
    }

    pub fn with_emergency_repair(mut self, emergency_repair: EmergencyRepair) -> Self {
        self.1.emergency_repair = Some(emergency_repair);
        self
    }

    pub fn with_replicas(mut self, replicas: Replicas) -> Self {
        self.1.replicas = Some(replicas);
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for ReconfigureBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
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
            dry_run: self.dry_run,
            emergency_repair: self.emergency_repair,
            replicas,
            primary_replica_tag,
            shards: self.shards,
        };

        opts.serialize(serializer)
    }
}
