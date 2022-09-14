use crate::Command;
use ql2::term::TermType;

use super::CmdOpts;

pub(crate) fn new(args: impl IndexWaitArg) -> Command {
    let mut command = Command::new(TermType::IndexWait);

    if let Some(args) = args.into_index_wait_opts() {
        command = args.add_to_cmd(command)
    }

    command
}

pub trait IndexWaitArg {
    fn into_index_wait_opts(self) -> Option<CmdOpts>;
}

impl IndexWaitArg for () {
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        None
    }
}

impl IndexWaitArg for &str {
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        let arg = Command::from_json(self);

        Some(CmdOpts::Single(arg))
    }
}

impl IndexWaitArg for Vec<&str> {
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        let args = self.into_iter().map(Command::from_json).collect();

        Some(CmdOpts::Many(args))
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{set_up, tear_down};
    use crate::types::IndexStatusResponse;
    use crate::Result;
    use crate::{prelude::*, Command, Session};

    #[tokio::test]
    async fn test_get_index_waited() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        generate_data(&conn, &table).await?;

        let indexes_waited: Vec<IndexStatusResponse> = table
            .clone()
            .index_wait(())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(indexes_waited.len() == 3);
        indexes_waited
            .iter()
            .for_each(|index_waited| assert!(index_waited.ready));

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_get_index_status_with_param() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        generate_data(&conn, &table).await?;

        let index_waited = table
            .clone()
            .index_wait("author")
            .run(&conn)
            .await?
            .unwrap()
            .parse::<Vec<IndexStatusResponse>>()?;

        let index_waited = index_waited.first().unwrap();

        assert!(index_waited.index == "author");
        assert!(index_waited.ready);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_get_index_status_with_params() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        generate_data(&conn, &table).await?;

        let indexes_waited: Vec<IndexStatusResponse> = table
            .clone()
            .index_wait(vec!["age", "name"])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(indexes_waited.len() == 2);
        indexes_waited
            .iter()
            .for_each(|index_waited| assert!(index_waited.ready));

        tear_down(conn, &table_name).await
    }

    async fn generate_data(conn: &Session, table: &Command) -> Result<()> {
        table.clone().index_create("author").run(conn).await?;
        table.clone().index_create("name").run(conn).await?;
        table.clone().index_create("age").run(conn).await?;
        Ok(())
    }
}
