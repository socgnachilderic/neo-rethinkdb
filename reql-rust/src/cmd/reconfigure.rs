use std::borrow::Cow;
use std::collections::HashMap;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::{Serialize, Serializer};

use crate::types::{EmergencyRepair, Replicas};
use crate::Command;

pub(crate) fn new(opts: ReconfigureOption) -> Command {
    Command::new(TermType::Reconfigure).with_opts(opts)
}

#[derive(Debug, Clone, Default, PartialEq, CommandOptions)]
#[non_exhaustive]
pub struct ReconfigureOption {
    pub dry_run: Option<bool>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
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

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::{ReconfigureResponse, Replicas};
    use crate::Result;

    use super::ReconfigureOption;

    #[tokio::test]
    async fn test_reconfigure_table() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let reconfigure_option = ReconfigureOption::default()
            .shards(2)
            .replicas(Replicas::Int(1));
        let response: ReconfigureResponse = table
            .reconfigure(reconfigure_option)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;
            
        assert!(response.reconfigured == 1);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
