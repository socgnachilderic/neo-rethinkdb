use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(value: impl Serialize) -> Command {
    let arg = Command::from_json(value);

    Command::new(TermType::SetInsert).with_arg(arg)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_set_insert_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [u8; 6] = r
            .expr([10, 20, 30, 40, 50])
            .set_insert(60)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == [10, 20, 30, 40, 50, 60]);

        Ok(())
    }
}
