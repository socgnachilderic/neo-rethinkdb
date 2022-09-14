use ql2::term::TermType;
use serde::Serialize;

use crate::{types::AnyParam, Command};

use super::CmdOpts;

pub(crate) fn new(args: impl EqArg) -> Command {
    args.into_eq_opts().add_to_cmd(Command::new(TermType::Eq))
}

pub trait EqArg {
    fn into_eq_opts(self) -> CmdOpts;
}

impl EqArg for AnyParam {
    fn into_eq_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
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
    use crate::spec::{set_up, tear_down, TABLE_NAMES};
    use crate::types::AnyParam;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_eq_data() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("title")
            .eq(AnyParam::new("title1"))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, TABLE_NAMES[0]).await
    }

    #[tokio::test]
    async fn test_eq_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.eq(vec![5, 5, 5]).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
