use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Minutes)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_minutes_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let minutes = r.now().minutes();
        let minutes1 = minutes.clone().value();
        let minutes2: u8 = minutes.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(minutes1 == minutes2);

        Ok(())
    }
}


