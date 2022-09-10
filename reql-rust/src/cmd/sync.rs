use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Sync)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::SyncResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_sync_ops() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let sync_response: SyncResponse = table.sync().run(&conn).await?.unwrap().parse()?;

        assert!(sync_response.synced == 1);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
