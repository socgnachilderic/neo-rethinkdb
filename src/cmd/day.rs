use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Day)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_day_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let day = r.now().day();
        let day1 = day.clone().value();
        let day2: u8 = day.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(day1 == day2);

        Ok(())
    }
}
