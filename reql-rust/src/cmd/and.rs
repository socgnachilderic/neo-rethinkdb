use ql2::term::TermType;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl AndArg) -> Command {
    args.into_and_opts().add_to_cmd(Command::new(TermType::And))
}

pub trait AndArg {
    fn into_and_opts(self) -> CmdOpts;
}

impl AndArg for bool {
    fn into_and_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl AndArg for Vec<bool> {
    fn into_and_opts(self) -> CmdOpts {
        let commands = self.iter().map(|arg| Command::from_json(arg)).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_and_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .and(vec![true, true, true])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
