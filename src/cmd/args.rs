use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(values: Vec<impl Serialize>) -> Command {
    let args = Command::from_json(values);

    Command::new(TermType::Args).with_arg(args)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Args<T>(pub(crate) T);

#[cfg(test)]
mod tests {
    use crate::{prelude::Converter, r, Result};

    #[tokio::test]
    async fn test_args_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data = vec![1, 2, 3];
        let response: Vec<u8> = r.args(data.clone()).run(&conn).await?.unwrap().parse()?;

        assert!(response == data);

        Ok(())
    }
}
