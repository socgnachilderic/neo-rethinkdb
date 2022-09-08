use crate::Command;
use ql2::term::TermType;

pub(crate) fn new(args: impl IndexStatusArg) -> Command {
    let mut command = Command::new(TermType::IndexStatus);
    let index_names = args.into_index_status_opts();

    if index_names.len() > 0 {
        for index_name in index_names {
            let arg = Command::from_json(index_name);
            command = command.with_arg(arg);
        }
    }

    command
}

pub trait IndexStatusArg {
    fn into_index_status_opts(self) -> Vec<String>;
}

impl IndexStatusArg for () {
    fn into_index_status_opts(self) -> Vec<String> {
        Vec::new()
    }
}

impl IndexStatusArg for &str {
    fn into_index_status_opts(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IndexStatusArg for Vec<&str> {
    fn into_index_status_opts(self) -> Vec<String> {
        self.iter().map(|index| index.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::{set_up, tear_down};
    use crate::types::IndexStatusResponse;
    use crate::Result;
    use crate::{prelude::*, Command, Session};

    #[tokio::test]
    async fn test_get_index_status() -> Result<()> {
        let (conn, table) = set_up("malik1").await?;
        generate_data(&conn, &table).await?;

        let index_status: Vec<IndexStatusResponse> = table
            .clone()
            .index_status(())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_status.len() == 3);

        tear_down(conn, "malik1").await
    }

    #[tokio::test]
    async fn test_get_index_status_with_param() -> Result<()> {
        let (conn, table) = set_up("malik2").await?;
        generate_data(&conn, &table).await?;

        let index_status: Vec<IndexStatusResponse> = table
            .clone()
            .index_status("author")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_status.len() == 1);
        assert!(index_status.first().unwrap().index == "author");

        tear_down(conn, "malik2").await
    }

    #[tokio::test]
    async fn test_get_index_status_with_params() -> Result<()> {
        let (conn, table) = set_up("malik3").await?;
        generate_data(&conn, &table).await?;

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

        tear_down(conn, "malik3").await
    }

    async fn generate_data(conn: &Session, table: &Command) -> Result<()> {
        table.clone().index_create("author").run(conn).await?;
        table.clone().index_create("name").run(conn).await?;
        table.clone().index_create("age").run(conn).await?;
        Ok(())
    }
}
