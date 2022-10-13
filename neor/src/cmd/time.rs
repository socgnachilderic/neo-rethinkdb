use ql2::term::TermType;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

use crate::arguments::Args;
use crate::types::timezone_to_string;
use crate::Command;

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
