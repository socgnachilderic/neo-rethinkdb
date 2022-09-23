use ql2::term::TermType;

use crate::{arguments::Args, Command};

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

impl<T> AndArg for Args<T>
where
    T: IntoIterator<Item = bool>,
{
    fn into_and_opts(self) -> CmdOpts {
        let commands = self.0.into_iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_and_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .and(args!([true, true, true]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
