use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl GtArg) -> Command {
    args.into_gt_opts().add_to_cmd(Command::new(TermType::Gt))
}

pub trait GtArg {
    fn into_gt_opts(self) -> CmdOpts;
}

impl<T: Serialize> GtArg for T {
    fn into_gt_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl<T: Serialize> GtArg for Args<Vec<T>> {
    fn into_gt_opts(self) -> CmdOpts {
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
    async fn test_gt_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("view")
            .gt(5)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_gt_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .gt(args!(vec![7, 6, 5]))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
