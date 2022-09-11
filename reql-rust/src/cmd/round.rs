use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl RoundArg) -> Command {
    let mut command = Command::new(TermType::Round);

    if let Some(arg) = args.into_round_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait RoundArg {
    fn into_round_opts(self) -> Option<Command>;
}

impl RoundArg for () {
    fn into_round_opts(self) -> Option<Command> {
        None
    }
}

impl RoundArg for f64 {
    fn into_round_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_round_data() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: u8 = r.round(12.345).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == 12);

        Ok(())
    }
}
