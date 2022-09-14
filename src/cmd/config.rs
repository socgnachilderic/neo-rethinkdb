use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Config)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::ConfigChangeValue;
    use crate::Result;

    #[tokio::test]
    async fn test_get_config_info() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], false).await?;
        let response: ConfigChangeValue = table.config().run(&conn).await?.unwrap().parse()?;

        assert!(response.name == TABLE_NAMES[0]);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
