use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::types::{AnyParam, Interleave};
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

impl UnionArg for AnyParam {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self.into()), Default::default())
    }
}

impl UnionArg for (Command, UnionOption) {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self.0), self.1)
    }
}

impl UnionArg for (Vec<Command>, UnionOption) {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Many(self.0), self.1)
    }
}

impl UnionArg for (AnyParam, UnionOption) {
    fn into_union_opts(self) -> (CmdOpts, UnionOption) {
        (CmdOpts::Single(self.0.into()), self.1)
    }
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct UnionOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interleave: Option<Interleave>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::AnyParam;
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
        let (conn, table) = set_up(TABLE_NAMES[1], true).await?;

        r.table_create(TABLE_NAMES[2]).run(&conn).await?;
        r.table(TABLE_NAMES[2])
            .insert(AnyParam::new(authors_data))
            .run(&conn)
            .await?;

        let data_obtained: Vec<AuthorPost> = table
            .union(r.table(TABLE_NAMES[2]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained.len() > 0);

        r.table_drop(TABLE_NAMES[2]).run(&conn).await?;
        tear_down(conn, TABLE_NAMES[1]).await
    }
}
