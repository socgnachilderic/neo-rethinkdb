use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Values)
}

#[cfg(test)]
mod tests {
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_values_fields() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained = table.get(1).values().run(&conn).await?.unwrap();

        assert!(data_obtained.is_array());

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
