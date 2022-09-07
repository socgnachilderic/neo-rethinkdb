use ql2::term::TermType;

use crate::Command;

pub fn make_db_create_command(db_name: &str) -> Command {
    let args = Command::from_json(db_name);

    Command::new(TermType::DbCreate).with_arg(args)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::types::DbResponse;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_create_db() -> Result<()> {
        let dbname = "zuma";
        let conn = r.connection().connect().await?;
        let db_created: DbResponse = r.db_create(dbname).run(&conn).await?.unwrap().parse();

        assert!(db_created.dbs_created == Some(1));

        r.db_drop(dbname).run(&conn).await?;
        Ok(())
    }
}
