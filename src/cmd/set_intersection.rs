use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(args: impl SetIntersectionArg) -> Command {
    Command::new(TermType::SetIntersection).with_arg(args.into_set_intersection_opts())
}

pub trait SetIntersectionArg {
    fn into_set_intersection_opts(self) -> Command;
}

impl<S, T> SetIntersectionArg for T
where
    S: Serialize,
    T: IntoIterator<Item = S> + Serialize,
{
    fn into_set_intersection_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SetIntersectionArg for Command {
    fn into_set_intersection_opts(self) -> Command {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_set_intersection_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 2] = r
            .expr([10, 20, 30, 40, 50])
            .set_intersection([20, 40])
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [20, 40]);

        Ok(())
    }
}
