use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, Conflict, Durability, ReturnChanges};
use crate::Command;

pub(crate) fn new(args: impl InsertArg) -> Command {
    let (arg, opts) = args.into_insert_opts();

    Command::new(TermType::Insert).with_arg(arg).with_opts(opts)
}

pub trait InsertArg {
    fn into_insert_opts(self) -> (Command, InsertOption);
}

impl<T> InsertArg for T
where
    T: Serialize,
{
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (Command::from_json(self), Default::default())
    }
}

impl InsertArg for Command {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (self, Default::default())
    }
}

impl<T> InsertArg for Args<(T, InsertOption)>
where
    T: Serialize,
{
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (Command::from_json(self.0 .0), self.0 .1)
    }
}

impl InsertArg for Args<(Command, InsertOption)> {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (self.0 .0, self.0 .1)
    }
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::arguments::ReturnChanges;
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::types::MutationResponse;
    use crate::{args, r, Result};

    use super::InsertOption;

    #[tokio::test]
    async fn test_insert_data() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table, table_name) = set_up(false).await?;
        let data_inserted: MutationResponse = table
            .clone()
            .insert(&data)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == 1);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_insert_many_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(false).await?;
        let data_inserted: MutationResponse = table
            .clone()
            .insert(&data)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == data.len());

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_insert_data_by_copy() -> Result<()> {
        let data = Post::get_many_data();
        let table_name2 = Uuid::new_v4().to_string();
        let (conn, table, table_name) = set_up(false).await?;

        r.table_create(table_name2.as_str()).run(&conn).await?;
        table.clone().insert(&data).run(&conn).await?;

        let data_inserted: MutationResponse = r
            .table(table_name2.as_str())
            .insert(table.clone())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == data.len());

        r.table_drop(table_name2.as_str()).run(&conn).await?;
        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_insert_data_with_opts() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table, table_name) = set_up(false).await?;
        let insert_options = InsertOption::default().return_changes(ReturnChanges::Bool(true));
        let data_inserted: MutationResponse = table
            .clone()
            .insert(args!(&data, insert_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!((&data_inserted).inserted == 1);
        let expected_data: Post = data_inserted
            .changes
            .unwrap()
            .first()
            .unwrap()
            .clone()
            .new_val
            .unwrap()
            .parse()?;
        assert!(expected_data == data);

        tear_down(conn, &table_name).await
    }
}
