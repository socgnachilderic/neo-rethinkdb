use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::IsEmpty)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_is_empty() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: bool = table.is_empty().run(&conn).await?.unwrap().parse()?;

        assert!(!data_obtained);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
