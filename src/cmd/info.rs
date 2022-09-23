use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Info)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::{InfoResponse, TypeOf};
    use crate::Result;

    #[tokio::test]
    async fn test_info_table() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        let data_obtained: InfoResponse = table.info().run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained.typ == TypeOf::Table);

        tear_down(conn, &table_name).await
    }
}
