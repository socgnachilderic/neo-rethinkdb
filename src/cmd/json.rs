use ql2::term::TermType;

use crate::Command;

pub(crate) fn new<T>(value: T) -> Command
where
    T: Into<String>,
{
    Command::new(TermType::Json).with_arg(Command::from_json(value.into()))
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_json_table() -> Result<()> {
        let data = [1, 2, 3];
        let conn = r.connection().connect().await?;
        let data_obtained: [u8; 3] = r.json("[1, 2, 3]").run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == data);

        Ok(())
    }
}
