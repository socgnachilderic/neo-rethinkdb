use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Sync)
}

#[cfg(test)]
mod tests {
    use crate::cmd::insert::InsertOption;
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down, Post, DATABASE_NAMES};
    use crate::types::{Durability, SyncResponse};
    use crate::Result;

    #[tokio::test]
    async fn test_sync_ops() -> Result<()> {
        let (conn, table) = set_up(DATABASE_NAMES[0]).await?;
        let data = Post::get_many_data();
        let insert_option = InsertOption::default().durability(Durability::Soft);
        table
            .clone()
            .insert((&data, insert_option))
            .run(&conn)
            .await?;

        let sync_response: SyncResponse = table.sync().run(&conn).await?.unwrap().parse()?;

        assert!(sync_response.synced == 1);

        tear_down(conn, DATABASE_NAMES[0]).await
    }
}
