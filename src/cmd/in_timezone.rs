use ql2::term::TermType;
use time::UtcOffset;

use crate::types::timezone_to_string;
use crate::Command;

pub(crate) fn new(timezone: UtcOffset) -> Command {
    Command::new(TermType::InTimezone).with_arg(Command::from_json(timezone_to_string(timezone)))
}

#[cfg(test)]
mod tests {
    use time::macros::offset;

    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_in_timezone_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let date_time = r.now().in_timezone(offset!(-08:00));
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 != time1);

        Ok(())
    }
}
