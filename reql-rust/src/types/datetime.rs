use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de, ser};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::macros::time;
use time::{format_description, Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::constants::{NANOS_PER_MSEC, NANOS_PER_SEC, TIMEZONE_FORMAT};
use crate::{cmd::run, Command};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Time {
    #[serde(rename = "$reql_type$")]
    reql_type: String,
    #[serde(with = "epoch_time")]
    epoch_time: String,
    timezone: String,
}

#[derive(Clone)]
pub struct DateTime(OffsetDateTime, pub(crate) Option<Command>);

impl DateTime {
    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<DateTime>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<DateTime>> {
        self.1
            .unwrap()
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DateTime>(arg)
    }

    pub(crate) fn now() -> Self {
        let offset_datetime = OffsetDateTime::now_utc();
        Self::default().create_datetime_command(Some(offset_datetime), Some(TermType::Now))
    }

    pub(crate) fn time(date: Date, timezone: UtcOffset, time: Option<time::Time>) -> Self {
        let mut primetive_datetime = PrimitiveDateTime::new(date, time!(0:00));

        if let Some(time) = time {
            primetive_datetime = primetive_datetime.replace_time(time);
        }

        let offset_datetime = primetive_datetime.assume_offset(timezone);
        Self::default().create_datetime_command(Some(offset_datetime), Some(TermType::Time))
    }

    pub(crate) fn epoch_time(timestamp: i64) -> crate::Result<Self> {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(timestamp)?;
        Ok(Self::default().create_datetime_command(Some(offset_datetime), Some(TermType::EpochTime)))
    }

    pub(crate) fn iso8601(
        iso_datetime: &str,
        default_timezone: Option<UtcOffset>,
    ) -> crate::Result<Self> {
        let mut datetime = iso_datetime.to_string();

        if let Some(timezone) = default_timezone {
            let timezone_format = format_description::parse(TIMEZONE_FORMAT)?;
            let timezone = timezone.format(&timezone_format)?;

            datetime = format!("{}{}", datetime, timezone);
        }

        let datetime = OffsetDateTime::parse(&datetime, &format_description::well_known::Rfc3339)?;
        Ok(Self::default().create_datetime_command(Some(datetime), Some(TermType::Iso8601)))
    }

    /// Return a new time object with a different timezone. While the time stays the same,
    /// the results returned by methods such as hours() will change since they take the timezone into account.
    /// The timezone argument has to be of the ISO 8601 format.
    ///
    /// ## Example
    ///
    /// Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::{json, Value};
    /// use time::macros::offset;
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.now().in_timezone(offset!(-08:00));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn in_timezone(&self, timezone: UtcOffset) -> Self {
        let datetime = self.0.clone().replace_offset(timezone);

        self.clone()
            .create_datetime_command(Some(datetime), Some(TermType::InTimezone))
    }

    /// Return the timezone of the time object.
    ///
    /// ## Example
    ///
    /// Return Timezone “-07:00”.
    ///
    /// ```
    /// use reql_rust::prelude::*;
    /// use reql_rust::{r, Result};
    /// use serde_json::{json, Value};
    /// use time::macros::offset;
    ///
    /// async fn example() -> Result<()> {
    ///     let session = r.connection().connect().await?;
    ///     let _ = r.now().in_timezone(offset!(-07:00)).timezone();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn timezone(&self) -> UtcOffset {
        self.0.offset()
    }

    pub fn during(&self, start_time: &DateTime, end_time: &DateTime) -> bool {
        self.le(start_time) && self.gt(end_time)
    }

    pub fn date(&self) -> Self {
        let datetime = self.0.clone().replace_time(time!(12:00));

        self.clone()
            .create_datetime_command(Some(datetime), Some(TermType::Date))
    }

    pub fn time_of_day(&self) -> u32 {
        let day: u32 = self.0.day().into();
        day * 60 * 60
    }

    fn create_datetime_command(
        mut self,
        offset_datetime: Option<OffsetDateTime>,
        term_type: Option<TermType>,
    ) -> Self {
        if let Some(term_type) = term_type {
            let command = Command::new(term_type);
            self.1 = Some(command);
        }

        if let Some(offset_datetime) = offset_datetime {
            self.0 = offset_datetime
        }

        self
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = Time::deserialize(deserializer)?;
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
        let (secs, msecs) = match time.epoch_time.split_once('.') {
            Some(parts) => parts,
            None => (time.epoch_time.as_str(), "0"),
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
        let dt = &self.0;
        let offset = dt.offset();
        let timezone = {
            let (hours, minutes, _) = offset.as_hms();
            format!(
                "{}{:02}:{:02}",
                if offset.is_negative() { '-' } else { '+' },
                hours.abs(),
                minutes.abs(),
            )
        };
        let time = Time {
            reql_type: "TIME".to_owned(),
            epoch_time: format!("{}.{:03}", dt.unix_timestamp(), dt.millisecond()),
            timezone,
        };
        time.serialize(serializer)
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

mod epoch_time {
    use super::*;

    pub fn serialize<S>(epoch_time: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match epoch_time.parse::<f64>() {
            Ok(timestamp) => serializer.serialize_f64(timestamp),
            Err(..) => Err(ser::Error::custom("invalid epoch timestamp")),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = f64::deserialize(deserializer)?;
        Ok(timestamp.to_string())
    }
}
