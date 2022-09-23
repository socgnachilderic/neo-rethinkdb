use ql2::term::TermType;

use crate::{arguments::Args, Command};

pub(crate) fn new(args: impl SplitArg) -> Command {
    let (arg1, arg2) = args.into_split_opts();
    let mut command = Command::new(TermType::Split);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg);
    }

    if let Some(arg1) = arg2 {
        command = command.with_arg(arg1);
    }

    command
}

pub trait SplitArg {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>);
}

impl SplitArg for () {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl SplitArg for &str {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(Command::from_json(self)), None)
    }
}

impl SplitArg for Command {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(self), None)
    }
}

impl SplitArg for Args<(&str, usize)> {
    fn into_split_opts(self) -> (Option<Command>, Option<Command>) {
        (
            Some(Command::from_json(self.0 .0)),
            Some(Command::from_json(self.0 .1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{args, prelude::Converter, r, Result};

    #[tokio::test]
    async fn test_split_ops() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data = ["foo".to_owned(), "bar".to_owned(), "bax".to_owned()];
        let response: [String; 3] = r
            .expr("foo bar bax")
            .split(())
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == data);

        Ok(())
    }

    #[tokio::test]
    async fn test_split_ops_entries() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data = [
            "12".to_owned(),
            "37".to_owned(),
            String::new(),
            "22".to_owned(),
            String::new(),
        ];
        let response: [String; 5] = r
            .expr("12,37,,22,")
            .split(",")
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == data);

        Ok(())
    }

    #[tokio::test]
    async fn test_split_ops_entries_limit() -> Result<()> {
        let conn = r.connection().connect().await?;
        let data = [
            "12".to_owned(),
            "37".to_owned(),
            String::new(),
            "22,".to_owned(),
        ];
        let response: [String; 4] = r
            .expr("12,37,,22,")
            .split(args!(",", 3))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response == data);

        Ok(())
    }
}
