use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Rebalance)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::RebalanceResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_rebalance_table() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: RebalanceResponse = table.rebalance().run(&conn).await?.unwrap().parse()?;

        assert!(response.rebalanced == 1);

        tear_down(conn, &table_name).await
    }
}
