use ql2::term::TermType;
use serde::Serialize;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl OffsetsOfArg) -> Command {
    Command::new(TermType::OffsetsOf).with_arg(args.into_offsets_of_opts())
}

pub trait OffsetsOfArg {
    fn into_offsets_of_opts(self) -> Command;
}

impl<T> OffsetsOfArg for T
where
    T: Serialize,
{
    fn into_offsets_of_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl OffsetsOfArg for Func {
    fn into_offsets_of_opts(self) -> Command {
        self.0
    }
}

impl OffsetsOfArg for Command {
    fn into_offsets_of_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_offset_of_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: Vec<usize> = r
            .expr(['a', 'b', 'c'])
            .offsets_of('c')
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.first() == Some(&2));

        Ok(())
    }
}
