use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Status)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::StatusResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_status_table() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let response: StatusResponse = table.status().run(&conn).await?.unwrap().parse()?;

        assert!(response.name.unwrap() == TABLE_NAMES[0]);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
