use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(arg: Func) -> Command {
    Command::new(TermType::ForEach).with_arg(arg.0)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::{set_up, tear_down};
    use crate::types::MutationResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_for_each_opts() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: MutationResponse = table
            .clone()
            .for_each(func!(|doc| table.get(doc.g("id")).delete(())))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.deleted == 5);

        tear_down(conn, &table_name).await
    }
}
