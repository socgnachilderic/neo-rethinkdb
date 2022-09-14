use ql2::term::TermType;

use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl AvgArg) -> Command {
    let (arg1, arg2) = args.into_avg_opts();
    let mut command = Command::new(TermType::Avg);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg)
    }

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command
}

pub trait AvgArg {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>);
}

impl AvgArg for () {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        (None, None)
    }
}

impl AvgArg for &str {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        let arg = Command::from_json(self);

        (None, Some(arg))
    }
}

impl AvgArg for Func {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        (None, Some(self.0))
    }
}

impl AvgArg for Command {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        (Some(self), None)
    }
}

impl AvgArg for (Command, &str) {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        let arg = Command::from_json(self.1);

        (Some(self.0), Some(arg))
    }
}

impl AvgArg for (Command, Func) {
    fn into_avg_opts(self) -> (Option<Command>, Option<Command>) {
        let Func(func) = self.1;

        (Some(self.0), Some(func))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::spec::{set_up, tear_down, Post};
    use crate::Result;

    #[tokio::test]
    async fn test_avg_data() -> Result<()> {
        let data: Vec<u8> = Post::get_many_data().iter().map(|post| post.view).collect();
        let avg = data.iter().sum::<u8>() as f32 / data.len() as f32;
        let (conn, table, table_name) = set_up(true).await?;
        let data_obtained: f32 = table.avg("view").run(&conn).await?.unwrap().parse()?;

        assert!(data_obtained == avg);

        tear_down(conn, table_name.as_str()).await
    }
}
