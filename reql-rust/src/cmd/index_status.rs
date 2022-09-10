use crate::Command;
use ql2::term::TermType;

use super::CmdOpts;

pub(crate) fn new(args: impl IndexStatusArg) -> Command {
    let mut command = Command::new(TermType::IndexStatus);
    let args = args.into_index_status_opts();

    if let Some(args) = args {
        command = args.add_to_cmd(command)
    }

    command
}

pub trait IndexStatusArg {
    fn into_index_status_opts(self) -> Option<CmdOpts>;
}

impl IndexStatusArg for () {
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        None
    }
}

impl IndexStatusArg for &str {
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        let arg = Command::from_json(self);

        Some(CmdOpts::Single(arg))
    }
}

impl IndexStatusArg for Vec<&str> {
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        let args = self
            .into_iter()
            .map(|arg| Command::from_json(arg))
            .collect();

        Some(CmdOpts::Many(args))
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::IndexStatusResponse;
    use crate::Result;
    use crate::{prelude::*, Command, Session};

    #[tokio::test]
    async fn test_get_index_status() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[1], false).await?;
        generate_index(&conn, &table).await?;

        let index_status: Vec<IndexStatusResponse> = table
            .clone()
            .index_status(())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_status.len() == 3);

        tear_down(conn, TABLE_NAMES[1]).await
    }

    #[tokio::test]
    async fn test_get_index_status_with_param() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[2], false).await?;
        generate_index(&conn, &table).await?;

        let index_status: Vec<IndexStatusResponse> = table
            .clone()
            .index_status("author")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_status.len() == 1);
        assert!(index_status.first().unwrap().index == "author");

        tear_down(conn, TABLE_NAMES[2]).await
    }

    #[tokio::test]
    async fn test_get_index_status_with_params() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[3], false).await?;
        generate_index(&conn, &table).await?;

        let index_status: Vec<IndexStatusResponse> = table
            .clone()
            .index_status(vec!["age", "name"])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_status.len() == 2);
        assert!(index_status.first().unwrap().index == "age");
        assert!(index_status.last().unwrap().index == "name");

        tear_down(conn, TABLE_NAMES[3]).await
    }

    async fn generate_index(conn: &Session, table: &Command) -> Result<()> {
        table.clone().index_create("author").run(conn).await?;
        table.clone().index_create("name").run(conn).await?;
        table.clone().index_create("age").run(conn).await?;

        Ok(())
    }
}
