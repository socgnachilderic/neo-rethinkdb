use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Date)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_date_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let datetime = r.now().date();
        let date1 = datetime.clone().value();
        let date2: Time = datetime.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(date1 == date2);

        Ok(())
    }
}
