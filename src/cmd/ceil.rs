use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl CeilArg) -> Command {
    let mut command = Command::new(TermType::Ceil);

    if let Some(arg) = args.into_ceil_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait CeilArg {
    fn into_ceil_opts(self) -> Option<Command>;
}

impl CeilArg for () {
    fn into_ceil_opts(self) -> Option<Command> {
        None
    }
}

impl CeilArg for Command {
    fn into_ceil_opts(self) -> Option<Command> {
        Some(self)
    }
}

impl CeilArg for f64 {
    fn into_ceil_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_ceil_data() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: u8 = r.ceil(12.345).run(&conn).await?.unwrap().parse()?;
        let data_obtained2: u8 = r.expr(12.345).ceil().run(&conn).await?.unwrap().parse()?;
        let data_obtained3: u8 = r.ceil(r.expr(12.345)).run(&conn).await?.unwrap().parse()?;

        assert!(
            data_obtained == 13
                && data_obtained == data_obtained2
                && data_obtained == data_obtained3
        );

        Ok(())
    }
}
