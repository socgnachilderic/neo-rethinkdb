use ql2::term::TermType;

use crate::{types::AnyParam, Command};

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

impl BranchArg for (AnyParam, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (None, CmdOpts::Single(self.0.into()), self.1.into())
    }
}

impl BranchArg for (Command, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (None, CmdOpts::Single(self.0), self.1)
    }
}

impl BranchArg for (Vec<AnyParam>, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        let true_action = self.0.into_iter().map(|arg| arg.into()).collect();

        (None, CmdOpts::Many(true_action), self.1.into())
    }
}

impl BranchArg for (Vec<Command>, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (None, CmdOpts::Many(self.0), self.1)
    }
}

impl BranchArg for (Command, AnyParam, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (Some(self.0), CmdOpts::Single(self.1.into()), self.2.into())
    }
}

impl BranchArg for (Command, Command, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (Some(self.0), CmdOpts::Single(self.1), self.2)
    }
}

impl BranchArg for (Command, Vec<AnyParam>, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        let true_action = self.1.into_iter().map(|arg| arg.into()).collect();

        (Some(self.0), CmdOpts::Many(true_action), self.2.into())
    }
}

impl BranchArg for (Command, Vec<Command>, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (Some(self.0), CmdOpts::Many(self.1), self.2)
    }
}

impl BranchArg for (AnyParam, AnyParam, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (
            Some(self.0.into()),
            CmdOpts::Single(self.1.into()),
            self.2.into(),
        )
    }
}

impl BranchArg for (AnyParam, Command, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (Some(self.0.into()), CmdOpts::Single(self.1), self.2)
    }
}

impl BranchArg for (AnyParam, Vec<AnyParam>, AnyParam) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        let true_action = self.1.into_iter().map(|arg| arg.into()).collect();

        (
            Some(self.0.into()),
            CmdOpts::Many(true_action),
            self.2.into(),
        )
    }
}

impl BranchArg for (AnyParam, Vec<Command>, Command) {
    fn into_branch_opts(self) -> (Option<Command>, CmdOpts, Command) {
        (Some(self.0.into()), CmdOpts::Many(self.1), self.2)
    }
}

// #[allow(array_into_iter)]
// #[allow(clippy::into_iter_on_ref)]
// impl<const N: usize> Arg for Args<([(Command, Command); N], Command)> {
//     fn arg(self) -> cmd::Arg<()> {
//         let Args((arr, false_action)) = self;
//         let mut query = Command::new(TermType::Branch);
//         // TODO remove the clone in Rust v1.53
//         for (test, true_action) in arr.into_iter().cloned() {
//             query = query.with_arg(test).with_arg(true_action);
//         }
//         query.with_arg(false_action).into_arg()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::prelude::Converter;
    use crate::types::AnyParam;
    use crate::{r, Result};

    #[tokio::test]
    async fn test_branch_data() -> Result<()> {
        let x = 10;
        let conn = r.connection().connect().await?;
        let response: String = r
            .branch((
                AnyParam::new(x > 5),
                AnyParam::new("big"),
                AnyParam::new("small"),
            ))
            .run(&conn)
            .await?
            .unwrap()
            .parse()?;

        assert!(response.eq("big"));

        Ok(())
    }
}
