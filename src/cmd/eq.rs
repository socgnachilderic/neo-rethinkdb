use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl EqArg) -> Command {
    args.into_eq_opts().add_to_cmd(Command::new(TermType::Eq))
}

pub trait EqArg {
    fn into_eq_opts(self) -> CmdOpts;
}

impl EqArg for Command {
    fn into_eq_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<T: Serialize> EqArg for Vec<T> {
    fn into_eq_opts(self) -> CmdOpts {
        let commands = self.iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::{r, Result};

    #[tokio::test]
    async fn test_eq_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("title")
            .eq(r.expr("title1"))
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
        let data_obtained: bool = r.eq(vec![5, 5, 5]).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
