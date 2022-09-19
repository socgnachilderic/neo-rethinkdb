use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Month)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_month_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let month = r.now().month();
        let month1 = month.clone().value();
        let month2: u8 = month.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(month1 == month2);

        Ok(())
    }
}
