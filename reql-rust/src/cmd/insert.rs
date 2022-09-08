use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Document;
use crate::types::{Conflict, Durability, ReturnChanges};
use crate::Command;

pub(crate) fn new(args: impl InsertArg) -> Command {
    let (arg, opts) = args.into_insert_opts();
    Command::new(TermType::Insert).with_arg(arg).with_opts(opts)
}

pub trait InsertArg {
    fn into_insert_opts(self) -> (Command, InsertOption);
}

impl<T: Document> InsertArg for T {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        let command = Command::from_json(self.get_document());

        (command, Default::default())
    }
}

impl InsertArg for Command {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (self, Default::default())
    }
}

impl<T: Document> InsertArg for (T, InsertOption) {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        let command = Command::from_json(self.0.get_document());

        (command, self.1)
    }
}

impl InsertArg for (Command, InsertOption) {
    fn into_insert_opts(self) -> (Command, InsertOption) {
        (self.0, self.1)
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
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post, DATABASE_NAMES};
    use crate::types::{ReturnChanges, WritingResponse};
    use crate::{r, Result};

    use super::InsertOption;

    #[tokio::test]
    async fn test_insert_data() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table) = set_up(DATABASE_NAMES[1]).await?;
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert(&data)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == 1);

        tear_down(conn, DATABASE_NAMES[1]).await
    }

    #[tokio::test]
    async fn test_insert_many_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(DATABASE_NAMES[2]).await?;
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert(&data)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == 2);

        tear_down(conn, DATABASE_NAMES[2]).await
    }

    #[tokio::test]
    async fn test_insert_data_by_copy() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table) = set_up(DATABASE_NAMES[3]).await?;

        r.table_create(DATABASE_NAMES[5]).run(&conn).await?;
        table.clone().insert(&data).run(&conn).await?;

        let data_inserted: WritingResponse<Post> = r
            .table(DATABASE_NAMES[5])
            .insert(table.clone())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_inserted.inserted == 2);

        r.table_drop("malik_backup3").run(&conn).await?;
        tear_down(conn, DATABASE_NAMES[3]).await
    }

    #[tokio::test]
    async fn test_insert_data_with_opts() -> Result<()> {
        let data = Post::get_one_data();
        let (conn, table) = set_up(DATABASE_NAMES[4]).await?;
        let data_inserted: WritingResponse<Post> = table
            .clone()
            .insert((
                &data,
                InsertOption::default().return_changes(ReturnChanges::Bool(true)),
            ))
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

        tear_down(conn, DATABASE_NAMES[4]).await
    }
}
