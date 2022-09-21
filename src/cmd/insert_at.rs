use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(offset: isize, value: impl Serialize) -> Command {
    let arg_offset = Command::from_json(offset);
    let arg_value = Command::from_json(value);

    Command::new(TermType::InsertAt)
        .with_arg(arg_offset)
        .with_arg(arg_value)
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_insert_at_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [String; 4] = r
            .expr(["Moussa", "Ali", "Fati"])
            .insert_at(1, "Alima")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == ["Moussa", "Alima", "Ali", "Fati"]);

        Ok(())
    }
}
