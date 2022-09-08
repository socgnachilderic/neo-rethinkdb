use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::IndexList)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::{set_up, tear_down, Result};

    #[tokio::test]
    async fn test_list_index() -> Result<()> {
        let (conn, table) = set_up("malik").await?;
        table.clone().index_create("author").run(&conn).await?;
        let index_list: Vec<String> = table
            .clone()
            .index_list()
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(index_list.len() > 0);

        tear_down(conn, "malik").await
    }
}
