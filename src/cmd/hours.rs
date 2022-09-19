use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Hours)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_hours_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let hours = r.now().hours();
        let hours1 = hours.clone().value();
        let hours2: u8 = hours.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(hours1 == hours2);

        Ok(())
    }
}

