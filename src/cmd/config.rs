use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Config)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::ConfigResponse;
    use crate::Result;

    #[tokio::test]
    async fn test_get_config_info() -> Result<()> {
        let (conn, table, table_name) = set_up(false).await?;
        let response: ConfigResponse = table.config().run(&conn).await?.unwrap().parse()?;

        assert!(response.name == table_name);

        tear_down(conn, table_name.as_str()).await
    }
}
