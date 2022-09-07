use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::types::{IdentifierFormat, ReadMode};
use crate::Command;

pub(crate) fn new(args: impl TableArg) -> Command {
    let (table_name, opts) = args.into_table_opts();
    let arg = Command::from_json(table_name);

    Command::new(TermType::Table).with_arg(arg).with_opts(opts)
}

pub trait TableArg {
    fn into_table_opts(self) -> (String, TableOption);
}

impl TableArg for &str {
    fn into_table_opts(self) -> (String, TableOption) {
        (self.to_string(), Default::default())
    }
}

impl TableArg for (&str, TableOption) {
    fn into_table_opts(self) -> (String, TableOption) {
        (self.0.to_string(), self.1)
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

/* impl<T: Unpin + Serialize + DeserializeOwned> TableBuilder<T> {
    pub fn do_(&self, func: Func) -> super::do_::DoBuilder {
        super::do_::DoBuilder::new(func)._with_parent(self.get_parent())
    }

    pub fn order_by(&self) -> super::order_by::OrderByBuilder<T> {
        super::order_by::OrderByBuilder::new()._with_parent(self.get_parent())
    }
    pub fn get_intersecting<A>(
        &self,
        geometry: &A,
        index: &'static str,
    ) -> super::get_intersecting::GetIntersectingBuilder<A>
    where
        A: ReqlOpsGeometry + Serialize,
    {
        super::get_intersecting::GetIntersectingBuilder::new(geometry, index)
            ._with_parent(self.get_parent())
    }

    pub fn get_nearest(&self, point: &Point, index: &'static str) -> super::get_nearest::GetNearestBuilder {
        super::get_nearest::GetNearestBuilder::new(point, index)._with_parent(self.get_parent())
    }

    pub fn grant(&self, username: &str) -> super::grant::GrantBuilder {
        super::grant::GrantBuilder::new(username)._with_parent(self.get_parent())
    }

    pub fn config(&self) -> super::config::ConfigBuilder {
        super::config::ConfigBuilder::new()._with_parent(self.get_parent())
    }

    pub fn rebalance(&self) -> super::rebalance::RebalanceBuilder {
        super::rebalance::RebalanceBuilder::new()._with_parent(self.get_parent())
    }

    pub fn reconfigure(&self) -> super::reconfigure::ReconfigureBuilder {
        super::reconfigure::ReconfigureBuilder::new()._with_parent(self.get_parent())
    }

    pub fn status(&self) -> super::status::StatusBuilder {
        super::status::StatusBuilder::new()._with_parent(self.get_parent())
    }

    pub fn wait(&self) -> super::wait::WaitBuilder {
        super::wait::WaitBuilder::new()._with_parent(self.get_parent())
    }
}
 */

// Sequence<Document<T>>
#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::cmd::table::TableOption;
    use crate::prelude::*;
    use crate::types::ReadMode;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_table() -> Result<()> {
        let conn = r.connection().connect().await?;
        let table: Vec<Value> = r
            .db("todo_app")
            .table("geo")
            .run(&conn)
            .await?
            .unwrap()
            .parse();

        assert!(table.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_table_with_options() -> Result<()> {
        let conn = r.connection().connect().await?;
        let table_options = TableOption::default().read_mode(ReadMode::Single);
        let table: Vec<Value> = r
            .db("todo_app")
            .table(("geo", table_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse();

        assert!(table.len() > 0);
        Ok(())
    }
}
