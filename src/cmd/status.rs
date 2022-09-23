use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Status)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::StatusResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_status_table() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: StatusResponse = table.status().run(&conn).await?.unwrap().parse()?;

        assert!(response.name.unwrap() == table_name);

        tear_down(conn, &table_name).await
    }
}
