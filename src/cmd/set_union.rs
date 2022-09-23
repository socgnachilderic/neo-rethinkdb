use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl SetUnionArg) -> Command {
    Command::new(TermType::SetUnion).with_arg(args.into_set_union_opts())
}

pub trait SetUnionArg {
    fn into_set_union_opts(self) -> Command;
}

impl<S, T> SetUnionArg for T
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    fn into_set_union_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SetUnionArg for Command {
    fn into_set_union_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_set_union_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 7] = r
            .expr([10, 20, 30, 40, 50])
            .set_union([60, 70])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [10, 20, 30, 40, 50, 60, 70]);

        Ok(())
    }
}
