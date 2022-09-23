use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Year)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_year_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let year = r.now().year();
        let year1 = year.clone().value();
        let year2: i32 = year.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(year1 == year2);

        Ok(())
    }
}
