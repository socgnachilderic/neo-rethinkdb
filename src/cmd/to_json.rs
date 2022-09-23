use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::ToJsonString)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::Result;

    #[tokio::test]
    async fn test_to_json_string() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: String = table.get(1).to_json().run(&conn).await?.unwrap().parse()?;

        assert!(!data_obtained.is_empty());

        tear_down(conn, &table_name).await
    }
}
