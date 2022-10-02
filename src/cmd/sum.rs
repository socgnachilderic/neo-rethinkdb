use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl SumArg) -> Command {
    let mut command = Command::new(TermType::Sum);

    if let Some(arg) = args.into_sum_opts() {
        command = command.with_arg(arg)
    }

    command
}

pub trait SumArg {
    fn into_sum_opts(self) -> Option<Command>;
}

impl SumArg for () {
    fn into_sum_opts(self) -> Option<Command> {
        None
    }
}

impl SumArg for &str {
    fn into_sum_opts(self) -> Option<Command> {
        let arg = Command::from_json(self);

        Some(arg)
    }
}

impl SumArg for Func {
    fn into_sum_opts(self) -> Option<Command> {
        Some(self.0)
    }
}

impl SumArg for Command {
    fn into_sum_opts(self) -> Option<Command> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_sum_data() -> Result<()> {
        let data: u8 = Post::get_many_data().iter().map(|post| post.view).sum();
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: u8 = table.sum("view").run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == data);

        tear_down(conn, &table_name).await
    }
}
