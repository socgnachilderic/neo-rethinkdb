use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl LeArg) -> Command {
    args.into_le_opts().add_to_cmd(Command::new(TermType::Le))
}

pub trait LeArg {
    fn into_le_opts(self) -> CmdOpts;
}

impl<T: Serialize> LeArg for T {
    fn into_le_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl<T: Serialize> LeArg for Args<Vec<T>> {
    fn into_le_opts(self) -> CmdOpts {
        let commands = self.0.iter().map(Command::from_json).collect();

        CmdOpts::Many(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down};
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_le_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("view")
            .le(10)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_le_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .le(args!(vec![5, 6, 7]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
