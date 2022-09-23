use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl DifferenceArg) -> Command {
    Command::new(TermType::Difference).with_arg(args.into_difference_opts())
}

pub trait DifferenceArg {
    fn into_difference_opts(self) -> Command;
}

impl<S, T> DifferenceArg for T
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    fn into_difference_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl DifferenceArg for Command {
    fn into_difference_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_difference_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 4] = r
            .expr([10, 20, 30, 40, 50])
            .difference([30, 70])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [10, 20, 40, 50]);

        Ok(())
    }
}
