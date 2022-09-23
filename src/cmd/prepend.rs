use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl PrependArg) -> Command {
    Command::new(TermType::Prepend).with_arg(args.into_prepend_opts())
}

pub trait PrependArg {
    fn into_prepend_opts(self) -> Command;
}

impl<T> PrependArg for T
where
    T: Serialize,
{
    fn into_prepend_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl PrependArg for Command {
    fn into_prepend_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_prepend_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 6] = r
            .expr([10, 20, 30, 40, 50])
            .prepend(0)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [0, 10, 20, 30, 40, 50]);

        Ok(())
    }
}
