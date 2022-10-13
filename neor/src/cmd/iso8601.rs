use ql2::term::TermType;
use time::{format_description, UtcOffset};

use crate::arguments::Args;
use crate::constants::TIMEZONE_FORMAT;
use crate::Command;

pub(crate) fn new(iso_datetime: &str) -> Command {
    Command::new(TermType::Iso8601).with_arg(Command::from_json(iso_datetime))
}

pub trait Iso8601 {
    fn into_iso8601_opts(self) -> crate::Result<String>;
}

impl<T> Iso8601 for T
where
    T: Into<String>,
{
    fn into_iso8601_opts(self) -> crate::Result<String> {
        Ok(self.into())
    }
}

impl<T> Iso8601 for Args<(T, UtcOffset)>
where
    T: Into<String>,
{
    fn into_iso8601_opts(self) -> crate::Result<String> {
        let timezone_format = format_description::parse(TIMEZONE_FORMAT)?;
        let timezone = self.0 .1.format(&timezone_format)?;

        Ok(format!("{}{}", self.0 .0.into(), timezone))
    }
}
