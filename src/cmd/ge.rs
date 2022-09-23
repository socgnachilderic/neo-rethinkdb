use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl GeArg) -> Command {
    args.into_ge_opts().add_to_cmd(Command::new(TermType::Ge))
}

pub trait GeArg {
    fn into_ge_opts(self) -> CmdOpts;
}

impl<T: Serialize> GeArg for T {
    fn into_ge_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl GeArg for Command {
    fn into_ge_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<S, T> GeArg for Args<T>
where
    S: Serialize,
    T: IntoIterator<Item = S>,
{
    fn into_ge_opts(self) -> CmdOpts {
        let commands = self.0.into_iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_ge_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("view")
            .ge(10)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_ge_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.ge(args!([7, 6, 5])).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
