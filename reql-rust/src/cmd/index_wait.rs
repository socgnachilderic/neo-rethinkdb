use crate::Command;
use ql2::term::TermType;

pub(crate) fn new(args: impl IndexWaitArg) -> Command {
    let mut command = Command::new(TermType::IndexWait);
    for index_name in args.into_index_wait_opts() {
        let arg = Command::from_json(index_name);
        command = command.with_arg(arg);
    }

    command
}

pub trait IndexWaitArg {
    fn into_index_wait_opts(self) -> Vec<String>;
}

impl IndexWaitArg for () {
    fn into_index_wait_opts(self) -> Vec<String> {
        Vec::new()
    }
}

impl IndexWaitArg for &str {
    fn into_index_wait_opts(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IndexWaitArg for Vec<&str> {
    fn into_index_wait_opts(self) -> Vec<String> {
        self.iter().map(|index| index.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::IndexStatusResponse;
    use crate::{prelude::*, Command, Session};
    use crate::{set_up, tear_down, Result};

    #[tokio::test]
    async fn test_get_index_waited() -> Result<()> {
        let (conn, table) = set_up("malik1").await?;
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

        tear_down(conn, "malik1").await
    }

    #[tokio::test]
    async fn test_get_index_status_with_param() -> Result<()> {
        let (conn, table) = set_up("malik2").await?;
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

        tear_down(conn, "malik2").await
    }

    #[tokio::test]
    async fn test_get_index_status_with_params() -> Result<()> {
        let (conn, table) = set_up("malik3").await?;
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

        tear_down(conn, "malik3").await
    }

    async fn generate_data(conn: &Session, table: &Command) -> Result<()> {
        table.clone().index_create("author").run(conn).await?;
        table.clone().index_create("name").run(conn).await?;
        table.clone().index_create("age").run(conn).await?;
        Ok(())
    }
}
