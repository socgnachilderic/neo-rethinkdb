use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::IndexList)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::Result;

    #[tokio::test]
    async fn test_list_index() -> Result<()> {
        let (conn, table) = set_up("malik", false).await?;
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
