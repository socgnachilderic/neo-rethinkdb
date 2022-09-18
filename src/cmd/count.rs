use ql2::term::TermType;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl CountArg) -> Command {
    let mut command = Command::new(TermType::Count);
    let (arg1, arg2) = args.into_count_arg();

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command
}

pub trait CountArg {
    fn into_count_arg(self) -> (Option<Command>, Option<Command>);
}

impl CountArg for () {
    fn into_count_arg(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl CountArg for Command {
    fn into_count_arg(self) -> (Option<Command>, Option<Command>) {
        (Some(self), None)
    }
}

impl CountArg for Func {
    fn into_count_arg(self) -> (Option<Command>, Option<Command>) {
        (Some(self.0), None)
    }
}

impl CountArg for Args<(Command, Func)> {
    fn into_count_arg(self) -> (Option<Command>, Option<Command>) {
        let Func(func) = self.0 .1;

        (Some(self.0 .0), Some(func))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_count_data() -> Result<()> {
        let data = Post::get_many_data();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: usize = table.count(()).run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == data.len());

        tear_down(conn, &table_name).await
    }
}
