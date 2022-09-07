use ql2::term::TermType;

use crate::Command;

pub fn make_db_list_command() -> Command {
    Command::new(TermType::DbList)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_list_db() -> Result<()> {
        let conn = r.connection().connect().await?;
        let db_list: Vec<String> = r.db_list().run(&conn).await?.unwrap().parse();

        assert!(db_list.len() > 0);
        Ok(())
    }
}
