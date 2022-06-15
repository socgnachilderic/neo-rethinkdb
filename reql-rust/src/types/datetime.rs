use std::ops::Deref;

use serde::{de, ser};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::macros::time;
use time::{format_description, Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::constants::{NANOS_PER_MSEC, NANOS_PER_SEC};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Time {
    #[serde(rename = "$reql_type$")]
    reql_type: String,
    #[serde(with = "epoch_time")]
    epoch_time: String,
    timezone: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DateTime(OffsetDateTime);

impl DateTime {
    pub(crate) fn now() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    pub(crate) fn time(date: Date, timezone: UtcOffset, time: Option<time::Time>) -> Self {
        let mut primetive_datetime = PrimitiveDateTime::new(date, time!(0:00));

        if let Some(time) = time {
            primetive_datetime = primetive_datetime.replace_time(time);
        }

        Self(primetive_datetime.assume_offset(timezone))
    }

    pub(crate) fn epoch_time(timestamp: i64) -> crate::Result<Self> {
        let time = OffsetDateTime::from_unix_timestamp(timestamp)?;
        Ok(Self(time))
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
        Ok(DateTime(dt))
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

impl Deref for DateTime {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OffsetDateTime> for DateTime {
    fn from(dt: OffsetDateTime) -> Self {
        Self(dt)
    }
}

impl From<DateTime> for OffsetDateTime {
    fn from(DateTime(dt): DateTime) -> Self {
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
