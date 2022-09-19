use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::ToIso8601)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_to_iso8601_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let iso8601 = r.now().to_iso8601();
        let iso8601_1 = iso8601.clone().value();
        let iso8601_2: String = iso8601.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(!iso8601_1.is_empty());
        assert!(!iso8601_2.is_empty());

        Ok(())
    }
}

