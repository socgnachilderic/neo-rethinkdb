use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use time::macros::time;
use time::{format_description, Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::arguments::Args;
use crate::constants::{NANOS_PER_MSEC, NANOS_PER_SEC, TIMEZONE_FORMAT};
use crate::{cmd, Command};

use super::response_with_cmd::ResponseWithCmd;
use super::Time;

#[derive(Clone)]
pub struct DateTime(pub OffsetDateTime, pub Option<Command>);

impl DateTime {
    pub(crate) fn now() -> Self {
        let offset_datetime = OffsetDateTime::now_utc();

        Self::default().create_datetime_command(Some(offset_datetime), Some(cmd::now::new()))
    }

    pub(crate) fn time(date: Date, timezone: UtcOffset, time: Option<time::Time>) -> Self {
        let mut primetive_datetime = PrimitiveDateTime::new(date, time!(0:00));
        let timezone_formated = timezone_to_string(timezone);

        if let Some(time) = time {
            primetive_datetime = primetive_datetime.replace_time(time);
        }

        let offset_datetime = primetive_datetime.assume_offset(timezone);

        Self::default().create_datetime_command(
            Some(offset_datetime),
            Some(cmd::time::new(
                date,
                timezone_formated,
                Some(primetive_datetime),
            )),
        )
    }

    pub(crate) fn epoch_time(timestamp: i64) -> crate::Result<Self> {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(timestamp)?;

        Ok(Self::default().create_datetime_command(
            Some(offset_datetime),
            Some(cmd::epoch_time::epoch_time(timestamp)),
        ))
    }

    pub(crate) fn iso8601(args: impl cmd::iso8601::Iso8601) -> crate::Result<Self> {
        let datetime = args.into_iso8601_opts()?;
        let command = cmd::iso8601::new(&datetime);
        let datetime = OffsetDateTime::parse(&datetime, &format_description::well_known::Rfc3339)?;

        Ok(Self::default().create_datetime_command(Some(datetime), Some(command)))
    }

    pub fn in_timezone(self, timezone: UtcOffset) -> Self {
        let datetime = self.0.replace_offset(timezone);

        self.clone().create_datetime_command(
            Some(datetime),
            Some(cmd::in_timezone::new(timezone).with_parent(self.cmd())),
        )
    }

    pub fn timezone(self) -> ResponseWithCmd<UtcOffset> {
        ResponseWithCmd(
            self.0.offset(),
            cmd::timezone::new().with_parent(self.cmd()),
        )
    }

    pub fn during(
        self,
        start_time: DateTime,
        end_time: DateTime,
        during_option: Option<cmd::during::DuringOption>,
    ) -> ResponseWithCmd<bool> {
        let is_verified = self.le(&end_time) && self.gt(&start_time);

        ResponseWithCmd(
            is_verified,
            cmd::during::new(Args((start_time, end_time, during_option))).with_parent(self.cmd()),
        )
    }

    pub fn date(self) -> Self {
        let datetime = self.0.replace_time(time!(00:00));

        self.clone().create_datetime_command(
            Some(datetime),
            Some(cmd::date::new().with_parent(self.cmd())),
        )
    }

    // FIXME not ready
    pub fn time_of_day(self) -> ResponseWithCmd<u32> {
        let day: u32 = self.0.day().into();

        ResponseWithCmd(
            day * 60 * 60,
            cmd::time_of_day::new().with_parent(self.cmd()),
        )
    }

    pub fn year(self) -> ResponseWithCmd<i32> {
        ResponseWithCmd(
            self.0.date().year(),
            cmd::year::new().with_parent(self.cmd()),
        )
    }

    pub fn month(self) -> ResponseWithCmd<u8> {
        ResponseWithCmd(
            self.0.date().month().into(),
            cmd::month::new().with_parent(self.cmd()),
        )
    }

    pub fn day(self) -> ResponseWithCmd<u8> {
        ResponseWithCmd(self.0.date().day(), cmd::day::new().with_parent(self.cmd()))
    }

    pub fn day_of_week(self) -> ResponseWithCmd<u8> {
        ResponseWithCmd(
            self.0.date().weekday().number_from_monday(),
            cmd::day_of_week::new().with_parent(self.cmd()),
        )
    }

    pub fn day_of_year(self) -> ResponseWithCmd<u16> {
        ResponseWithCmd(
            self.0.date().ordinal(),
            cmd::day_of_year::new().with_parent(self.cmd()),
        )
    }

    pub fn hours(self) -> ResponseWithCmd<u8> {
        ResponseWithCmd(
            self.0.time().hour(),
            cmd::hours::new().with_parent(self.cmd()),
        )
    }

    pub fn minutes(self) -> ResponseWithCmd<u8> {
        ResponseWithCmd(
            self.0.time().minute(),
            cmd::minutes::new().with_parent(self.cmd()),
        )
    }

    pub fn seconds(self) -> u8 {
        self.0.time().second()
    }

    pub fn to_iso8601(self) -> String {
        self.0
            .format(&format_description::well_known::Rfc3339)
            .unwrap()
    }

    pub fn to_epoch_time(self) -> i64 {
        self.0.unix_timestamp()
    }

    fn create_datetime_command(
        mut self,
        offset_datetime: Option<OffsetDateTime>,
        command: Option<Command>,
    ) -> Self {
        if let Some(command) = command {
            self.1 = Some(command);
        }

        if let Some(offset_datetime) = offset_datetime {
            self.0 = offset_datetime
        }

        self
    }

    pub fn cmd(self) -> Command {
        Command::from(self)
    }

    pub fn value(self) -> Time {
        Time::from(self)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = Time::deserialize(deserializer)?;
        let epoch_time = time.epoch_time.to_string();
        let format = match format_description::parse("[offset_hour]:[offset_minute]") {
            Ok(fmt) => fmt,
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        let offset = match UtcOffset::parse(&time.timezone, &format) {
            Ok(offset) => offset,
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        let (secs, msecs) = match epoch_time.split_once('.') {
            Some(parts) => parts,
            None => (epoch_time.as_str(), "0"),
        };
        let secs = match secs.parse::<i128>() {
            Ok(secs) => match secs.checked_mul(NANOS_PER_SEC) {
                Some(secs) => secs,
                None => {
                    return Err(de::Error::custom("seconds to nanosecond overflow"));
                }
            },
            Err(..) => {
                return Err(de::Error::custom("invalid epoch time seconds"));
            }
        };
        // RethinkDB timestamps have millisecond precision so we need
        // to convert the milliseconds to nanoseconds first
        let msecs = match msecs.parse::<i128>() {
            Ok(int) => {
                let msecs = match msecs.len() {
                    3 => int,
                    2 => int * 10,
                    1 => int * 100,
                    _ => {
                        return Err(de::Error::custom("invalid epoch milliseconds"));
                    }
                };
                match msecs.checked_mul(NANOS_PER_MSEC) {
                    Some(msecs) => msecs,
                    None => {
                        return Err(de::Error::custom("millisecond to nanosecond overflow"));
                    }
                }
            }
            Err(..) => {
                return Err(de::Error::custom("invalid epoch time milliseconds"));
            }
        };
        let timestamp = match secs.checked_add(msecs) {
            Some(timestamp) => timestamp,
            None => {
                return Err(de::Error::custom("timestamp addition overflow"));
            }
        };
        let dt = match OffsetDateTime::from_unix_timestamp_nanos(timestamp) {
            Ok(date_time) => date_time.to_offset(offset),
            Err(error) => {
                return Err(de::Error::custom(error));
            }
        };
        Ok(DateTime(dt, None))
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Time::from(self.to_owned()).serialize(serializer)
    }
}

impl Default for DateTime {
    fn default() -> Self {
        let offset_datetime = OffsetDateTime::now_utc();

        Self(offset_datetime, None)
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DateTime").field(&self.0).finish()
    }
}

impl Deref for DateTime {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for DateTime {}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Hash for DateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<OffsetDateTime> for DateTime {
    fn from(dt: OffsetDateTime) -> Self {
        Self(dt, None)
    }
}

impl From<DateTime> for OffsetDateTime {
    fn from(DateTime(dt, _): DateTime) -> Self {
        dt
    }
}

impl From<DateTime> for Command {
    fn from(date_time: DateTime) -> Self {
        date_time.1.unwrap()
    }
}

impl From<DateTime> for Time {
    fn from(date_time: DateTime) -> Self {
        Self::from(date_time.0)
    }
}

pub fn timezone_to_string(timezone: UtcOffset) -> String {
    if timezone.is_utc() {
        String::from("Z")
    } else {
        let format = format_description::parse(TIMEZONE_FORMAT).unwrap();
        timezone.format(&format).unwrap()
    }
}
