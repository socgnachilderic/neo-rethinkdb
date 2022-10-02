use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl CountArg) -> Command {
    let mut command = Command::new(TermType::Count);

    if let Some(arg) = args.into_count_arg() {
        command = command.with_arg(arg)
    }

    command
}

pub trait CountArg {
    fn into_count_arg(self) -> Option<Command>;
}

impl CountArg for () {
    fn into_count_arg(self) -> Option<Command> {
        None
    }
}

impl CountArg for Command {
    fn into_count_arg(self) -> Option<Command> {
        Some(self)
    }
}

impl CountArg for Func {
    fn into_count_arg(self) -> Option<Command> {
        Some(self.0)
    }
}

impl<T> CountArg for Args<T>
where
    T: Serialize,
{
    fn into_count_arg(self) -> Option<Command> {
        Some(Command::from_json(self.0))
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
