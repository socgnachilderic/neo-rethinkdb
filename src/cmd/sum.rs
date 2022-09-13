use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl SumArg) -> Command {
    let (arg1, arg2) = args.into_sum_opts();
    let mut command = Command::new(TermType::Sum);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command
}

pub trait SumArg {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>);
}

impl SumArg for () {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl SumArg for &str {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        let arg = Command::from_json(self);

        (None, Some(arg))
    }
}

impl SumArg for Func {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(self.0))
    }
}

impl SumArg for Command {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(self), None)
    }
}

impl SumArg for (Command, &str) {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        let arg = Command::from_json(self.1);

        (Some(self.0), Some(arg))
    }
}

impl SumArg for (Command, Func) {
    fn into_sum_opts(self) -> (Option<Command>, Option<Command>) {
        let Func(func) = self.1;

        (Some(self.0), Some(func))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post, TABLE_NAMES};
    use crate::Result;

    #[tokio::test]
    async fn test_sum_data() -> Result<()> {
        let data: u8 = Post::get_many_data().iter().map(|post| post.view).sum();
        let (conn, table) = set_up(TABLE_NAMES[0], true).await?;
        let data_obtained: u8 = table.sum("view").run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == data);

        tear_down(conn, TABLE_NAMES[0]).await
    }
}
