use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::types::{DateTime, Status};
use crate::Command;

pub(crate) fn new(args: impl DuringArg) -> Command {
    let (arg1, arg2, opts) = args.into_during_opts();

    Command::new(TermType::During)
        .with_arg(arg1)
        .with_arg(arg2)
        .with_opts(opts)
}

pub trait DuringArg {
    fn into_during_opts(self) -> (Command, Command, DuringOption);
}

impl DuringArg for Args<(DateTime, DateTime)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0.into(), self.0 .1.into(), Default::default())
    }
}

impl DuringArg for Args<(DateTime, DateTime, DuringOption)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0.into(), self.0 .1.into(), self.0 .2)
    }
}

impl DuringArg for Args<(DateTime, DateTime, Option<DuringOption>)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (
            self.0 .0.into(),
            self.0 .1.into(),
            self.0 .2.unwrap_or_default(),
        )
    }
}

impl DuringArg for Args<(Command, Command)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0, self.0 .1, Default::default())
    }
}

impl DuringArg for Args<(Command, Command, DuringOption)> {
    fn into_during_opts(self) -> (Command, Command, DuringOption) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, CommandOptions)]
pub struct DuringOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_bound: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_bound: Option<Status>,
}

#[cfg(test)]
mod tests {
    use time::macros::{date, offset};

    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_during_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let start_date = r.time(date!(2022 - 08 - 01), offset!(UTC), None);
        let end_date = r.time(date!(2022 - 12 - 31), offset!(UTC), None);

        let datetime = r.epoch_time(1661990400)?;

        let (response1, cmd) = datetime
            .clone()
            .during(start_date.clone(), end_date.clone(), None);
        let response2: bool = cmd.run(&conn).await?.unwrap().parse()?;
        let response3: bool = datetime
            .cmd()
            .during(args!(start_date, end_date))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response1 == response2 && response1 == response3);

        Ok(())
    }
}
