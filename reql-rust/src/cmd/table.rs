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
    pub fn index_create(&self, index_name: &str) -> super::index_create::IndexCreateBuilder {
        super::index_create::IndexCreateBuilder::new(index_name)._with_parent(self.get_parent())
    }

    pub fn index_drop(&self, index_name: &str) -> super::index_drop::IndexDropBuilder {
        super::index_drop::IndexDropBuilder::new(index_name)._with_parent(self.get_parent())
    }

    pub fn index_list(&self) -> super::index_list::IndexListBuilder {
        super::index_list::IndexListBuilder::new()._with_parent(self.get_parent())
    }

    pub fn index_rename(
        &self,
        old_index_name: &str,
        new_index_name: &str,
    ) -> super::index_rename::IndexRenameBuilder {
        super::index_rename::IndexRenameBuilder::new(old_index_name, new_index_name)
            ._with_parent(self.get_parent())
    }

    pub fn index_status(&self) -> super::index_status::IndexStatusBuilder {
        super::index_status::IndexStatusBuilder::new()._with_parent(self.get_parent())
    }

    pub fn index_wait(&self) -> super::index_wait::IndexWaitBuilder {
        super::index_wait::IndexWaitBuilder::new()._with_parent(self.get_parent())
    }

    pub fn set_write_hook(&self, func: Func) -> super::set_write_hook::SetWriteHookBuilder {
        super::set_write_hook::SetWriteHookBuilder::new(func)._with_parent(self.get_parent())
    }

    pub fn get_write_hook(&self) -> super::get_write_hook::GetWriteBuilder {
        super::get_write_hook::GetWriteBuilder::new()._with_parent(self.get_parent())
    }

    pub fn insert(&self, document: &T) -> super::insert::InsertBuilder<T> {
        super::insert::InsertBuilder::new(document)._with_parent(self.get_parent())
    }

    pub fn insert_many(&self, documents: &[T]) -> super::insert::InsertBuilder<T> {
        assert!(documents.len() > 0);
        super::insert::InsertBuilder::new_many(documents)._with_parent(self.get_parent())
    }

    pub fn sync(&self) -> super::sync::SyncBuilder {
        super::sync::SyncBuilder::new()._with_parent(self.get_parent())
    }

    pub fn get(&self, primary_key: impl Serialize) -> super::get::GetBuilder<Option<Document<T>>> {
        super::get::GetBuilder::new(primary_key)._with_parent(self.get_parent())
    }

    pub fn get_all(
        &self,
        values: &[impl Serialize],
    ) -> super::get_all::GetAllBuilder<Sequence<Document<T>>> {
        super::get_all::GetAllBuilder::new(values)._with_parent(self.get_parent())
    }

    pub fn between(
        &self,
        lower_key: impl Serialize,
        upper_key: impl Serialize,
    ) -> super::between::BetweenBuilder<Sequence<Document<T>>> {
        super::between::BetweenBuilder::new(lower_key, upper_key)._with_parent(self.get_parent())
    }

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
