use ql2::term::TermType;
use serde::Serialize;

use crate::{arguments::Args, Command};

use super::CmdOpts;

pub(crate) fn new(args: impl EqArg) -> Command {
    args.into_eq_opts().add_to_cmd(Command::new(TermType::Eq))
}

pub trait EqArg {
    fn into_eq_opts(self) -> CmdOpts;
}

impl<T> EqArg for T
where
    T: Serialize,
{
    fn into_eq_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl EqArg for Command {
    fn into_eq_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<S, T> EqArg for Args<T>
where
    S: Serialize,
    T: IntoIterator<Item = S>,
{
    fn into_eq_opts(self) -> CmdOpts {
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
    async fn test_eq_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("title")
            .eq("title1")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_eq_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.eq(args!([5, 5, 5])).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
