use std::borrow::Cow;
use std::collections::HashMap;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::{Serialize, Serializer};

use crate::arguments::{Durability, Replicas};
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl TableCreateArg) -> Command {
    let (args, opts) = args.into_table_create_opts();

    args.add_to_cmd(Command::new(TermType::TableCreate))
        .with_opts(opts)
}

pub trait TableCreateArg {
    fn into_table_create_opts(self) -> (CmdOpts, TableCreateOption);
}

impl TableCreateArg for &str {
    fn into_table_create_opts(self) -> (CmdOpts, TableCreateOption) {
        let arg = Command::from_json(self);

        (CmdOpts::Single(arg), Default::default())
    }
}

impl TableCreateArg for (&str, TableCreateOption) {
    fn into_table_create_opts(self) -> (CmdOpts, TableCreateOption) {
        let arg = Command::from_json(self.0);

        (CmdOpts::Single(arg), self.1)
    }
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::cmd::table_create::TableCreateOption;
    use crate::types::DbResponse;
    use crate::{prelude::*, Session};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_create_table() -> Result<()> {
        let table_name = Uuid::new_v4().to_string();
        let conn = r.connection().connect().await?;
        let table_created: DbResponse = r
            .table_create(table_name.as_str())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        drop_table(&table_name, table_created, &conn).await
    }

    #[tokio::test]
    async fn test_create_table_with_options() -> Result<()> {
        let table_name = Uuid::new_v4().to_string();
        let conn = r.connection().connect().await?;
        let table_options = TableCreateOption::default().primary_key("id");
        let table_created = r
            .db("test")
            .table_create((table_name.as_str(), table_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        drop_table(&table_name, table_created, &conn).await
    }

    async fn drop_table(table_name: &str, table_created: DbResponse, conn: &Session) -> Result<()> {
        assert!(table_created.tables_created > Some(0));
        r.table_drop(table_name).run(conn).await?;
        Ok(())
    }
}
