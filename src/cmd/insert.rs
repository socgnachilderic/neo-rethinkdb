use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::{AnyParam, Conflict, Durability, ReturnChanges};
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl InsertArg) -> Command {
    let (args, opts) = args.into_insert_opts();

    args.add_to_cmd(Command::new(TermType::Insert))
        .with_opts(opts)
}

pub trait InsertArg {
    fn into_insert_opts(self) -> (CmdOpts, InsertOption);
}

impl InsertArg for AnyParam {
    fn into_insert_opts(self) -> (CmdOpts, InsertOption) {
        (CmdOpts::Single(self.into()), Default::default())
    }
}

impl InsertArg for (AnyParam, InsertOption) {
    fn into_insert_opts(self) -> (CmdOpts, InsertOption) {
        (CmdOpts::Single(self.0.into()), self.1)
    }
}

impl InsertArg for Command {
    fn into_insert_opts(self) -> (CmdOpts, InsertOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl InsertArg for (Command, InsertOption) {
    fn into_insert_opts(self) -> (CmdOpts, InsertOption) {
        (CmdOpts::Single(self.0), self.1)
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

    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post};
    use crate::types::{AnyParam, ReturnChanges, WritingResponse};
    use crate::{r, Result};

    use super::InsertOption;

    #[tokio::test]
    async fn test_insert_data() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table, table_name) = set_up(false).await?;
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert(AnyParam::new(&data))
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
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert(AnyParam::new(&data))
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
        table
            .clone()
            .insert(AnyParam::new(&data))
            .run(&conn)
            .await?;

        let data_inserted: WritingResponse<Post> = r
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
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert((AnyParam::new(&data), insert_options))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!((&data_inserted).inserted == 1);
        let expected_data = data_inserted
            .changes
            .unwrap()
            .first()
            .unwrap()
            .clone()
            .new_val;
        assert!(expected_data == Some(data));

        tear_down(conn, &table_name).await
    }
}
