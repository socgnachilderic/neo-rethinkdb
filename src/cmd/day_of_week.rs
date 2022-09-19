use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::DayOfWeek)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_day_of_week_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let day_of_week = r.now().day_of_week();
        let day_of_week1 = day_of_week.clone().value();
        let day_of_week2: u8 = day_of_week.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(day_of_week1 == day_of_week2);

        Ok(())
    }
}
