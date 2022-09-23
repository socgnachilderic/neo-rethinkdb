use ql2::term::TermType;

use crate::Command;

pub(crate) fn epoch_time(timestamp: i64) -> Command {
    Command::new(TermType::EpochTime).with_arg(Command::from_json(timestamp))
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_time_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let date_time = r.epoch_time(531360000)?;
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 == time1);

        Ok(())
    }
}
