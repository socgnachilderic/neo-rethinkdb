use ql2::term::TermType;

use crate::{arguments::Args, Command};

pub(crate) fn new(args: impl RangeArg) -> Command {
    let (arg1, arg2) = args.into_range_opts();
    let mut command = Command::new(TermType::Range);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command
}

pub trait RangeArg {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>);
}

impl RangeArg for () {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl RangeArg for isize {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(Command::from_json(self)))
    }
}

impl RangeArg for Args<(isize, isize)> {
    fn into_range_opts(self) -> (Option<Command>, Option<Command>) {
        (
            Some(Command::from_json(self.0 .0)),
            Some(Command::from_json(self.0 .1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_range_data() -> Result<()> {
        let data = [0, 1, 2, 3];
        let data2 = [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5];
        let conn = r.connection().connect().await?;
        let response: [isize; 4] = r.range(4).run(&conn).await?.unwrap().parse()?;
        let response2: [isize; 4] = r.range(()).limit(4).run(&conn).await?.unwrap().parse()?;
        let response3: [isize; 11] = r.range(args!(-5, 6)).run(&conn).await?.unwrap().parse()?;

        assert!(response == data);
        assert!(response2 == data);
        assert!(response3 == data2);

        Ok(())
    }
}
