use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(selector: impl Serialize) -> Command {
    let arg = Command::from_json(selector);

    Command::new(TermType::HasFields).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::Result;

    #[tokio::test]
    async fn test_has_fields() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .has_fields("title")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }
}
