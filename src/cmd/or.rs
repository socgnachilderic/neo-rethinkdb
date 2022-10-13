use ql2::term::TermType;

use crate::{arguments::Args, Command};

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

impl OrArg for Command {
    fn into_or_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<T> OrArg for Args<T>
where
    T: IntoIterator<Item = bool>,
{
    fn into_or_opts(self) -> CmdOpts {
        let commands = self.0.into_iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_or_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .or(args!([true, false]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
