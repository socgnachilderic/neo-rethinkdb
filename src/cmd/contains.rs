use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl ContainsArg) -> Command {
    args.into_contains_opts()
        .add_to_cmd(Command::new(TermType::Contains))
}

pub trait ContainsArg {
    fn into_contains_opts(self) -> CmdOpts;
}

impl<T> ContainsArg for T
where
    T: Serialize,
{
    fn into_contains_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl ContainsArg for Func {
    fn into_contains_opts(self) -> CmdOpts {
        CmdOpts::Single(self.0)
    }
}

impl ContainsArg for Command {
    fn into_contains_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<S, T> ContainsArg for Args<T>
where
    S: Into<Command>,
    T: IntoIterator<Item = S>,
{
    fn into_contains_opts(self) -> CmdOpts {
        CmdOpts::Many(self.0.into_iter().map(|cmd| cmd.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_contains_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: bool = r
            .expr(["red", "green", "blue"])
            .contains("green")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response);

        Ok(())
    }
}
