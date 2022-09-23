use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl SetDifferenceArg) -> Command {
    Command::new(TermType::SetDifference).with_arg(args.into_set_difference_opts())
}

pub trait SetDifferenceArg {
    fn into_set_difference_opts(self) -> Command;
}

impl<S, T> SetDifferenceArg for T
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    fn into_set_difference_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SetDifferenceArg for Command {
    fn into_set_difference_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_set_difference_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 3] = r
            .expr([10, 20, 30, 40, 50])
            .set_difference([20, 40])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [10, 30, 50]);

        Ok(())
    }
}
