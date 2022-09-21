use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

pub(crate) fn new(offset: isize, value: impl Serialize) -> Command {
    let arg_offset = Command::from_json(offset);
    let arg_value = Command::from_json(value);

    Command::new(TermType::ChangeAt)
        .with_arg(arg_offset)
        .with_arg(arg_value)
}

#[cfg(test)]
mod tests {
    use crate::{prelude::Converter, r, Result};

    #[tokio::test]
    async fn test_change_at_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [String; 3] = r
            .expr(["Moussa", "Ali", "Fati"])
            .change_at(1, "Alima")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

            assert!(response == ["Moussa", "Alima", "Fati"]);
            
        Ok(())
    }
}
