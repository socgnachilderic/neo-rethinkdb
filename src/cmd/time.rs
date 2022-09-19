use ql2::term::TermType;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

use crate::{arguments::Args, types::timezone_to_string, Command};

pub(crate) fn new(
    offset_datetime: OffsetDateTime,
    timezone_formated: String,
    with_time: bool,
) -> Command {
    let date = offset_datetime.date();
    let month: u8 = date.month().into();
    let mut command = Command::new(TermType::Time);
    command = command.with_arg(Command::from_json(date.year()));
    command = command.with_arg(Command::from_json(month));
    command = command.with_arg(Command::from_json(date.day()));

    if with_time {
        let time = offset_datetime.time();

        command = command.with_arg(Command::from_json(time.hour()));
        command = command.with_arg(Command::from_json(time.minute()));
        command = command.with_arg(Command::from_json(time.second()));
    }

    command.with_arg(Command::from_json(timezone_formated))
}

pub trait TimeArg {
    fn into_time_opts(self) -> (OffsetDateTime, String, bool);
}

impl TimeArg for Args<(Date, UtcOffset)> {
    fn into_time_opts(self) -> (OffsetDateTime, String, bool) {
        let (offset_datetime, timezone_formated) =
            make_time(self.0 .0, time::macros::time!(0:00), self.0 .1);

        (offset_datetime, timezone_formated, false)
    }
}

impl TimeArg for Args<(Date, Time, UtcOffset)> {
    fn into_time_opts(self) -> (OffsetDateTime, String, bool) {
        let (offset_datetime, timezone_formated) = make_time(self.0 .0, self.0 .1, self.0 .2);

        (offset_datetime, timezone_formated, true)
    }
}

fn make_time(date: Date, time: Time, timezone: UtcOffset) -> (OffsetDateTime, String) {
    let timezone_formated = timezone_to_string(timezone);
    let primetive_datetime = PrimitiveDateTime::new(date, time);
    let offset_datetime = primetive_datetime.assume_offset(timezone);

    (offset_datetime, timezone_formated)
}

#[cfg(test)]
mod tests {
    use time::macros::{date, offset, time};

    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_time_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let date = date!(1986 - 11 - 3);
        let timezone = offset!(+01:00);
        let time = time!(09:30:40);

        let date_time = r.time(args!(date, time, timezone));
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 == time1);

        Ok(())
    }
}
