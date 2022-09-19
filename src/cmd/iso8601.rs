use ql2::term::TermType;
use time::{format_description, UtcOffset};

use crate::{arguments::Args, constants::TIMEZONE_FORMAT, Command};

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

#[cfg(test)]
mod tests {
    use time::macros::offset;

    use crate::prelude::Converter;
    use crate::types::Time;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_iso8601_ops() -> Result<()> {
        let conn = r.connection().connect().await?;

        let date_time = r.iso8601("1986-11-03T08:30:00-07:00")?;
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 == time1);

        Ok(())
    }

    #[tokio::test]
    async fn test_iso8601_ops_with_default_timezone() -> Result<()> {
        let conn = r.connection().connect().await?;

        let date_time = r.iso8601(args!("1986-11-03T08:30:00", offset!(+01:00)))?;
        let time1 = date_time.clone().value();
        let time2: Time = date_time.cmd().run(&conn).await?.unwrap().parse()?;

        assert!(time2 == time1);

        Ok(())
    }
}
