use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl NeArg) -> Command {
    args.into_ne_opts().add_to_cmd(Command::new(TermType::Ne))
}

pub trait NeArg {
    fn into_ne_opts(self) -> CmdOpts;
}

impl<T> NeArg for T
where
    T: Serialize,
{
    fn into_ne_opts(self) -> CmdOpts {
        CmdOpts::Single(Command::from_json(self))
    }
}

impl NeArg for Command {
    fn into_ne_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl<S, T> NeArg for Args<T>
where
    S: Serialize,
    T: IntoIterator<Item = S>,
{
    fn into_ne_opts(self) -> CmdOpts {
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
    async fn test_ne_data() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: bool = table
            .get(1)
            .g("title")
            .ne("title")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        tear_down(conn, &table_name).await
    }

    #[tokio::test]
    async fn test_ne_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.ne(args!([5, 6, 7])).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
