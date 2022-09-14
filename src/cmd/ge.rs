use ql2::term::TermType;
use serde::Serialize;

use crate::{types::AnyParam, Command};

use super::CmdOpts;

pub(crate) fn new(args: impl GeArg) -> Command {
    args.into_ge_opts().add_to_cmd(Command::new(TermType::Ge))
}

pub trait GeArg {
    fn into_ge_opts(self) -> CmdOpts;
}

impl GeArg for AnyParam {
    fn into_ge_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl<T: Serialize> GeArg for Vec<T> {
    fn into_ge_opts(self) -> CmdOpts {
        let commands = self.iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::types::AnyParam;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_ge_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("view")
            .ge(AnyParam::new(10))
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
        let data_obtained: bool = r.ge(vec![7, 6, 5]).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
