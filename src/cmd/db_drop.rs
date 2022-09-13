use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(db_name: &str) -> Command {
    let arg = Command::from_json(db_name);

    Command::new(TermType::DbDrop).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::types::DbResponse;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_drop_db() -> Result<()> {
        let dbname = "zuma";
        let conn = r.connection().connect().await?;
        r.db_create(dbname).run(&conn).await?;

        let db_dropped: DbResponse = r.db_drop(dbname).run(&conn).await?.unwrap().parse()?;

        assert!(db_dropped.dbs_dropped == Some(1));
        Ok(())
    }
}
