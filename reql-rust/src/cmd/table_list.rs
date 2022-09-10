use crate::Command;
use ql2::term::TermType;

pub(crate) fn new() -> Command {
    Command::new(TermType::TableList)
}
#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_list_table() -> Result<()> {
        let conn = r.connection().connect().await?;
        let db_list: Vec<String> = r.table_list().run(&conn).await?.unwrap().parse()?;

        assert!(db_list.len() > 0);
        Ok(())
    }
}
