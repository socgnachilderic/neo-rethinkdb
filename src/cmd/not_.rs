use ql2::term::TermType;

use crate::Command;

// TODO use std::ops::Not
pub(crate) fn new(args: impl NotArg) -> Command {
    let mut command = Command::new(TermType::Not);

    if let Some(arg) = args.into_not_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait NotArg {
    fn into_not_opts(self) -> Option<Command>;
}

impl NotArg for () {
    fn into_not_opts(self) -> Option<Command> {
        None
    }
}

impl NotArg for bool {
    fn into_not_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_not_data_r() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r.not(false).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained);

        Ok(())
    }

    #[tokio::test]
    async fn test_not_data() -> Result<()> {
        let object = vec!["id", "id1", "title", "title1"];
        let conn = r.connection().connect().await?;
        let data_obtained: bool = r
            .object(object)
            .has_fields("content")
            .not_()
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(data_obtained);

        Ok(())
    }
}
