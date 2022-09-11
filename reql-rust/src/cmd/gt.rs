use ql2::term::TermType;
use serde::Serialize;

use crate::{types::AnyParam, Command};

use super::CmdOpts;

pub(crate) fn new(args: impl GtArg) -> Command {
    args.into_gt_opts().add_to_cmd(Command::new(TermType::Gt))
}

pub trait GtArg {
    fn into_gt_opts(self) -> CmdOpts;
}

impl GtArg for AnyParam {
    fn into_gt_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl<T: Serialize> GtArg for Vec<T> {
    fn into_gt_opts(self) -> CmdOpts {
        let commands = self.iter().map(|arg| Command::from_json(arg)).collect();

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
    async fn test_gt_data() -> Result<()> {
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("view")
            .gt(AnyParam::new(5))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, TABLE_NAMES[0]).await
    }

    #[tokio::test]
    async fn test_gt_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.gt(vec![7, 6, 5]).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
