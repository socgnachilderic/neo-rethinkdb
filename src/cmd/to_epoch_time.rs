use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::ToEpochTime)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_to_epoch_time_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let time = r.now().to_epoch_time();
        let time1 = time.clone().value();
        let time2: f64 = time.cmd().run(&conn).await?.unwrap().parse()?;
        
        assert!(time1.is_normal());
        assert!(time2.is_normal());

        Ok(())
    }
}

