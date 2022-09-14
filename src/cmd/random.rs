use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl RandomArg) -> Command {
    let (arg1, arg2, opts) = args.into_random_opts();
    let mut command = Command::new(TermType::Random);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait RandomArg {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption);
}

impl RandomArg for () {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (None, None, Default::default())
    }
}

impl RandomArg for isize {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for f64 {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (Some(Command::from_json(self)), None, Default::default())
    }
}

impl RandomArg for (isize, isize) {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(Command::from_json(self.0)),
            Some(Command::from_json(self.1)),
            Default::default(),
        )
    }
}

impl RandomArg for (f64, f64) {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(Command::from_json(self.0)),
            Some(Command::from_json(self.1)),
            Default::default(),
        )
    }
}

impl RandomArg for (isize, isize, RandomOption) {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(Command::from_json(self.0)),
            Some(Command::from_json(self.1)),
            self.2,
        )
    }
}

impl RandomArg for (f64, f64, RandomOption) {
    fn into_random_opts(self) -> (Option<Command>, Option<Command>, RandomOption) {
        (
            Some(Command::from_json(self.0)),
            Some(Command::from_json(self.1)),
            self.2,
        )
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, CommandOptions,
)]
pub struct RandomOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    use super::RandomOption;

    #[tokio::test]
    async fn test_random_data() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained1: f64 = r.random(()).run(&conn).await?.unwrap().parse()?;
        let data_obtained2: isize = r.random(100).run(&conn).await?.unwrap().parse()?;
        let data_obtained3: f64 = r
            .random((-100.52, -10.71, RandomOption::default().float(true)))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained1.is_normal());
        assert!(data_obtained2 >= 0);
        assert!(data_obtained3.is_normal());

        Ok(())
    }
}
