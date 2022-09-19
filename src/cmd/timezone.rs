use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Timezone)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_timezone_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let timezone = r.now().timezone();
        let timezone2: String = timezone.clone().cmd().run(&conn).await?.unwrap().parse()?;

        assert!(timezone.value().to_string() != timezone2);

        Ok(())
    }
}
