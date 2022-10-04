use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::{Args, Interleave};
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl UnionArg) -> Command {
    let (args, opts) = args.into_union_opts();

    args.add_to_cmd(Command::new(TermType::Union))
        .with_opts(opts)
}

pub trait UnionArg {
    fn into_union_opts(self) -> (CmdOpts, UnionOption);
}

impl UnionArg for Command {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl UnionArg for Vec<Command> {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Many(self), Default::default())
    }
}

impl UnionArg for Args<(Command, UnionOption)> {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self.0 .0), self.0 .1)
    }
}

impl UnionArg for Args<(Vec<Command>, UnionOption)> {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Many(self.0 .0), self.0 .1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct UnionOption {
    /// The optional `interleave` argument controls 
    /// how the sequences will be merged:
    /// - `Interleave::Bool(true)`: results will be mixed together; 
    /// this is the fastest setting, but ordering of elements is not guaranteed. 
    /// (This is the default.)
    /// - `Interleave::Bool(false)`: input sequences will be appended to one another, left to right.
    /// - `Interleave::FieldName(field_name)`: a string will be taken as the name of a field 
    /// to perform a merge-sort on. The input sequences must be ordered **before** being passed to `union`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interleave: Option<Interleave>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use uuid::Uuid;

    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::{r, Result};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct AuthorPost {
        id: Option<u8>,
        first_name: Option<String>,
        last_name: Option<String>,
        title: Option<String>,
        content: Option<String>,
        view: Option<u8>,
    }

    #[tokio::test]
    async fn test_union_data() -> Result<()> {
        let authors_data = json!([
            {"id": 1, "first_name": "john", "last_name": "doe"},
            {"id": 2, "first_name": "juan", "last_name": "don"},
            {"id": 3, "first_name": "jean", "last_name": "dupont"}
        ]);
        let table_name2 = Uuid::new_v4().to_string();
        let (conn, table, table_name) = set_up(true).await?;

        r.table_create(table_name2.as_str()).run(&conn).await?;
        r.table(table_name2.as_str())
            .insert(authors_data)
            .run(&conn)
            .await?;

        let data_obtained: Vec<AuthorPost> = table
            .union(r.table(table_name2.as_str()))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.len() > 0);

        r.table_drop(table_name2.as_str()).run(&conn).await?;
        tear_down(conn, &table_name).await
    }
}
