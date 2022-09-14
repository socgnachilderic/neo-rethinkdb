use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(table_name: &str) -> Command {
    let args = Command::from_json(table_name);

    Command::new(TermType::TableDrop).with_arg(args)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::prelude::*;
    use crate::types::DbResponse;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_drop_table() -> Result<()> {
        let table_name = Uuid::new_v4().to_string();
        let conn = r.connection().connect().await?;

        r.table_create(table_name.as_str()).run(&conn).await?;

        let table_dropped: DbResponse = r
            .table_drop(table_name.as_str())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(table_dropped.tables_dropped > Some(0));

        Ok(())
    }
}
