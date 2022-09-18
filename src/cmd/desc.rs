use ql2::term::TermType;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl DescArg) -> Command {
    Command::new(TermType::Desc).with_arg(args.into_desc_opts())
}

pub trait DescArg {
    fn into_desc_opts(self) -> Command;
}

impl<T> DescArg for T
where
    T: Into<T> + Serialize,
{
    fn into_desc_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl DescArg for Func {
    fn into_desc_opts(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::spec::*;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_desc_ops() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response: Vec<Post> = table
            .order_by(args!(r.expr("view"), r.desc("title")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.len() > 1);

        tear_down(conn, &table_name).await
    }
}
