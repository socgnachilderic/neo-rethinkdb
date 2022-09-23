use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl AppendArg) -> Command {
    Command::new(TermType::Append).with_arg(args.into_append_opts())
}

pub trait AppendArg {
    fn into_append_opts(self) -> Command;
}

impl<T> AppendArg for T
where
    T: Serialize,
{
    fn into_append_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl AppendArg for Command {
    fn into_append_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_append_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 6] = r
            .expr([10, 20, 30, 40, 50])
            .append(100)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [10, 20, 30, 40, 50, 100]);

        Ok(())
    }
}
