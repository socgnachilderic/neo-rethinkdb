use ql2::term::TermType;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

use super::CmdOpts;

pub(crate) fn new(args: impl DoArg) -> Command {
    args.into_do_opts()
        .add_to_cmd(Command::new(TermType::Funcall))
}

pub trait DoArg {
    fn into_do_opts(self) -> CmdOpts;
}

impl DoArg for Command {
    fn into_do_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl DoArg for Func {
    fn into_do_opts(self) -> CmdOpts {
        CmdOpts::Single(self.0)
    }
}

impl DoArg for Args<(Command, Command)> {
    fn into_do_opts(self) -> CmdOpts {
        CmdOpts::Many(vec![self.0 .0, self.0 .1])
    }
}

impl DoArg for Args<(Command, Func)> {
    fn into_do_opts(self) -> CmdOpts {
        let Func(func) = self.0 .1;

        CmdOpts::Many(vec![self.0 .0, func])
    }
}

impl<const N: usize> DoArg for Args<([Command; N], Func)> {
    fn into_do_opts(self) -> CmdOpts {
        let Func(func) = self.0 .1;
        let mut args = self.0 .0.to_vec();

        args.push(func);
        CmdOpts::Many(args)
    }
}

impl<const N: usize> DoArg for Args<([Command; N], Command)> {
    fn into_do_opts(self) -> CmdOpts {
        let mut args = self.0 .0.to_vec();

        args.push(self.0 .1);
        CmdOpts::Many(args)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::prelude::*;
    use crate::spec::*;
    use crate::{r, Result};

    #[tokio::test]
    #[ignore = "not work"]
    async fn test_do_opts() -> Result<()> {
        let (conn, table, table_name) = set_up(true).await?;
        let response = table
            .get(1)
            .do_(func!(|post| post.g("view").add(r.var(5))))
            .run(&conn)
            .await;

        dbg!(&response);

        tear_down(conn, &table_name).await
    }
}
