use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::Seconds)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_seconds_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let seconds = r.now().seconds();
        let seconds1 = seconds.clone().value();
        let seconds2: f64 = seconds.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(seconds1.is_normal());
        assert!(seconds2.is_normal());

        Ok(())
    }
}
