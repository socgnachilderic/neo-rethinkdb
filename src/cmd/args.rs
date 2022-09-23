use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new<T, S>(values: T) -> Command
where
    S: Serialize,
    T: AsRef<[S]> + Serialize,
{
    let args = Command::from_json(values);

    Command::new(TermType::Args).with_arg(args)
}

#[cfg(test)]
mod tests {
    use crate::{prelude::Converter, r, Result};

    #[tokio::test]
    async fn test_args_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data = vec![1, 2, 3];
        let response: Vec<u8> = r.args(&data).run(&conn).await?.unwrap().parse()?;

        assert!(response == data);

        Ok(())
    }
}
