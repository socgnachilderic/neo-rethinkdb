use serde::{Deserialize, Serialize};
use time::{format_description, OffsetDateTime, UtcOffset};

use crate::{
    constants::{NANOS_PER_MSEC, NANOS_PER_SEC},
    ReqlDriverError, ReqlError, Result,
};

use super::ReqlType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Time {
    #[serde(rename = "$reql_type$")]
    pub reql_type: ReqlType,
    pub epoch_time: f64,
    pub timezone: String,
}

impl Time {
    pub fn new(epoch_time: f64, timezone: String) -> Self {
        Self {
            epoch_time,
            timezone,
            reql_type: ReqlType::Time,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.to_offset_date_time().is_ok()
    }

    pub fn to_offset_date_time(&self) -> Result<OffsetDateTime> {
        let epoch_time = self.epoch_time.to_string();
        let format = match format_description::parse("[offset_hour]:[offset_minute]") {
            Ok(fmt) => fmt,
            Err(error) => {
                return Err(ReqlError::from(error));
            }
        };
        let offset = match UtcOffset::parse(&self.timezone, &format) {
            Ok(offset) => offset,
            Err(error) => {
                return Err(ReqlError::from(error));
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
                    return Err(ReqlError::Driver(ReqlDriverError::Time(
                        "seconds to nanosecond overflow".to_owned(),
                    )));
                }
            },
            Err(..) => {
                return Err(ReqlError::Driver(ReqlDriverError::Time(
                    "invalid epoch time seconds".to_owned(),
                )));
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
                        return Err(ReqlError::Driver(ReqlDriverError::Time(
                            "invalid epoch milliseconds".to_owned(),
                        )));
                    }
                };
                match msecs.checked_mul(NANOS_PER_MSEC) {
                    Some(msecs) => msecs,
                    None => {
                        return Err(ReqlError::Driver(ReqlDriverError::Time(
                            "millisecond to nanosecond overflow".to_owned(),
                        )));
                    }
                }
            }
            Err(..) => {
                return Err(ReqlError::Driver(ReqlDriverError::Time(
                    "invalid epoch time milliseconds".to_owned(),
                )));
            }
        };
        let timestamp = match secs.checked_add(msecs) {
            Some(timestamp) => timestamp,
            None => {
                return Err(ReqlError::Driver(ReqlDriverError::Time(
                    "timestamp addition overflow".to_owned(),
                )));
            }
        };

        let dt = match OffsetDateTime::from_unix_timestamp_nanos(timestamp) {
            Ok(date_time) => date_time.to_offset(offset),
            Err(error) => {
                return Err(ReqlError::from(error));
            }
        };

        Ok(dt)
    }
}

impl Eq for Time {}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        let dt = self.to_offset_date_time().unwrap();
        let dt2 = other.to_offset_date_time().unwrap();

        dt.eq(&dt2)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let dt = self.to_offset_date_time().unwrap();
        let dt2 = other.to_offset_date_time().unwrap();

        dt.partial_cmp(&dt2)
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let dt = self.to_offset_date_time().unwrap();
        let dt2 = other.to_offset_date_time().unwrap();

        dt.cmp(&dt2)
    }
}

impl From<OffsetDateTime> for Time {
    fn from(dt: OffsetDateTime) -> Self {
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

        let epoch_time = format!("{}.{:03}", dt.unix_timestamp(), dt.millisecond())
            .parse()
            .unwrap();

        Self::new(epoch_time, timezone)
    }
}
