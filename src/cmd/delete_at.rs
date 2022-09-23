use ql2::term::TermType;

use crate::arguments::Args;
use crate::Command;

pub(crate) fn new(args: impl DeleteAtArg) -> Command {
    let (start_offset, end_offset) = args.into_delete_at_opts();
    let mut command = Command::new(TermType::DeleteAt).with_arg(start_offset);

    if let Some(end_offset) = end_offset {
        command = command.with_arg(end_offset);
    }

    command
}

pub trait DeleteAtArg {
    fn into_delete_at_opts(self) -> (Command, Option<Command>);
}

impl DeleteAtArg for isize {
    fn into_delete_at_opts(self) -> (Command, Option<Command>) {
        (Command::from_json(self), None)
    }
}

impl DeleteAtArg for Args<(isize, isize)> {
    fn into_delete_at_opts(self) -> (Command, Option<Command>) {
        (
            Command::from_json(self.0 .0),
            Some(Command::from_json(self.0 .1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    const DATA: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];

    #[tokio::test]
    async fn test_delete_at_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let response: [char; 5] = r
            .expr(&DATA)
            .delete_at(1)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        let response2: [char; 5] = r
            .expr(&DATA)
            .delete_at(-2)
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        let response3: [char; 4] = r
            .expr(&DATA)
            .delete_at(args!(1, 3))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == ['a', 'c', 'd', 'e', 'f']);
        assert!(response2 == ['a', 'b', 'c', 'd', 'f']);
        assert!(response3 == ['a', 'd', 'e', 'f']);

        Ok(())
    }
}
