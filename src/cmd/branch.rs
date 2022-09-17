use ql2::term::TermType;

use crate::{arguments::Args, Command};

use super::CmdOpts;

pub(crate) fn new(args: impl BranchArg) -> Command {
    let (test, true_action, false_action) = args.into_branch_opts();
    let mut command = Command::new(TermType::Branch);

    if let Some(arg) = test {
        command = command.with_arg(arg)
    }

    true_action.add_to_cmd(command).with_arg(false_action)
}

pub trait BranchArg {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command);
}

impl BranchArg for Args<(Command, Command)> {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (None, CmdOpts::Single(self.0 .0), self.0 .1)
    }
}

impl<const N: usize> BranchArg for Args<(Command, [(Command, Command); N], Command)> {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        let actions = self
            .0
             .1
            .into_iter()
            .flat_map(|(test, true_action)| [test, true_action])
            .collect();
        (Some(self.0 .0), CmdOpts::Many(actions), self.0 .2)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::{args, r, Result};

    #[tokio::test]
    async fn test_branch_data() -> Result<()> {
        let x = 10;
        let conn = r.connection().connect().await?;
        let response: String = r
            .branch(r.var(x > 5), args!(r.var("big"), r.var("small")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.eq("big"));

        Ok(())
    }

    #[tokio::test]
    async fn test_branch_data_with_infix() -> Result<()> {
        let x = 10;
        let conn = r.connection().connect().await?;
        let response: String = r
            .expr(x > 5)
            .branch(args!(r.var("big"), r.var("small")))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.eq("big"));

        Ok(())
    }
}
