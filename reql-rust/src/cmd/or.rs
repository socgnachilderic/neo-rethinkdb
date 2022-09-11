use ql2::term::TermType;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl OrArg) -> Command {
    args.into_or_opts().add_to_cmd(Command::new(TermType::Or))
}

pub trait OrArg {
    fn into_or_opts(self) -> CmdOpts;
}

impl OrArg for bool {
    fn into_or_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl OrArg for Vec<bool> {
    fn into_or_opts(self) -> CmdOpts {
        let commands = self.iter().map(|arg| Command::from_json(arg)).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_or_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.or(vec![true, false]).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
