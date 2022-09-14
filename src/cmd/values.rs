use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Values)
}

#[cfg(test)]
mod tests {
    use crate::spec::{set_up, tear_down};
    use crate::Result;

    #[tokio::test]
    async fn test_values_fields() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained = table.get(1).values().run(&conn).await?.unwrap();

        assert!(data_obtained.is_array());

        tear_down(conn, &table_name).await
    }
}
