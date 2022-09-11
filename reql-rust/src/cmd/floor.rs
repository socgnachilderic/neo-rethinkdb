use ql2::term::TermType;

use crate::Command;

pub(crate) fn new(args: impl FloorArg) -> Command {
    let mut command = Command::new(TermType::Floor);

    if let Some(arg) = args.into_floor_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait FloorArg {
    fn into_floor_opts(self) -> Option<Command>;
}

impl FloorArg for () {
    fn into_floor_opts(self) -> Option<Command> {
        None
    }
}

impl FloorArg for f64 {
    fn into_floor_opts(self) -> Option<Command> {
        Some(Command::from_json(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_floor_data() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data_obtained: i8 = r.floor(-12.345).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == -13);

        Ok(())
    }
}
