use ql2::term::TermType;
use time::{Date, PrimitiveDateTime};

use crate::Command;

pub(crate) fn new(
    date: Date,
    timezone_formated: String,
    time: Option<PrimitiveDateTime>,
) -> Command {
    let month: u8 = date.month().into();
    let mut command = Command::new(TermType::Time);
    command = command.with_arg(Command::from_json(date.year()));
    command = command.with_arg(Command::from_json(month));
    command = command.with_arg(Command::from_json(date.day()));

    if let Some(primetive_datetime) = time {
        command = command.with_arg(Command::from_json(primetive_datetime.hour()));
        command = command.with_arg(Command::from_json(primetive_datetime.minute()));
        command = command.with_arg(Command::from_json(primetive_datetime.second()));
    }

    command.with_arg(Command::from_json(timezone_formated))
}

#[cfg(test)]
mod tests {
    use time::macros::{date, offset, time};

    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_time_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let date = date!(1986 - 11 - 3);
        let timezone = offset!(+01:00);
        let time = time!(09:30:40);

        let date_time = r.time(date, timezone, Some(time));
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 == time1);

        Ok(())
    }
}
