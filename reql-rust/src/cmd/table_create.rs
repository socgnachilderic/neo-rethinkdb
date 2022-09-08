use std::borrow::Cow;
use std::collections::HashMap;

use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::{Serialize, Serializer};

use crate::types::{Durability, Replicas};
use crate::Command;

pub(crate) fn new(args: impl TableCreateArg) -> Command {
    let (table_name, opts) = args.into_table_create_opts();
    let arg = Command::from_json(table_name);

    Command::new(TermType::TableCreate)
        .with_arg(arg)
        .with_opts(opts)
}

pub trait TableCreateArg {
    fn into_table_create_opts(self) -> (String, TableCreateOption);
}

impl TableCreateArg for &str {
    fn into_table_create_opts(self) -> (String, TableCreateOption) {
        (self.to_string(), Default::default())
    }
}

impl TableCreateArg for (&str, TableCreateOption) {
    fn into_table_create_opts(self) -> (String, TableCreateOption) {
        (self.0.to_string(), self.1)
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
    use crate::cmd::table_create::TableCreateOption;
    use crate::types::DbResponse;
    use crate::{prelude::*, Session};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_create_table() -> Result<()> {
        let table_name: &str = "malik1";
        let conn = r.connection().connect().await?;
        let table_created: DbResponse = r
            .table_create(table_name)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        drop_table(table_name, table_created, &conn).await
    }

    #[tokio::test]
    async fn test_create_table_with_options() -> Result<()> {
        let table_name: &str = "malik2";
        let conn = r.connection().connect().await?;
        let table_options = TableCreateOption::default().primary_key("id");
        let table_created = r
            .db("test")
            .table_create((table_name, table_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        drop_table(table_name, table_created, &conn).await
    }

    async fn drop_table(table_name: &str, table_created: DbResponse, conn: &Session) -> Result<()> {
        assert!(table_created.tables_created > Some(0));
        r.table_drop(table_name).run(conn).await?;
        Ok(())
    }
}
