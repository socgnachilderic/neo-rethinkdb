use ql2::term::TermType;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl AscArg) -> Command {
    Command::new(TermType::Asc).with_arg(args.into_asc_opts())
}

pub trait AscArg {
    fn into_asc_opts(self) -> Command;
}

impl<T> AscArg for T
where
    T: Into<T> + Serialize,
{
    fn into_asc_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl AscArg for Func {
    fn into_asc_opts(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::*;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_asc_ops() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: Vec<Post> = table
            .order_by(args!(r.expr("view"), r.asc("title")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;
            
        assert!(response.len() > 1);

        tear_down(conn, &table_name).await
    }
}
