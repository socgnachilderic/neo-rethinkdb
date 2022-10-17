use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl BranchArg) -> Command {
    let (test, true_action, false_action) = args.into_branch_opts();
    let mut command = Command::new(TermType::Branch);

    if let Some(arg) = test {
        command = command.with_arg(arg.to_cmd())
    }

    true_action
        .add_to_cmd(command)
        .with_arg(false_action.to_cmd())
}

pub trait BranchArg {
    fn into_branch_opts(self) -> (Option<CommandArg>, CmdOpts, CommandArg);
}

impl<S, T> BranchArg for Args<(S, T)>
where
    S: Into<CommandArg>,
    T: Into<CommandArg>,
{
    fn into_branch_opts(self) -> (Option<CommandArg>, CmdOpts, CommandArg) {
        (
            None,
            CmdOpts::Single(self.0 .0.into().to_cmd()),
            self.0 .1.into(),
        )
    }
}

impl<V, T, T1, T2, F> BranchArg for Args<(V, T, F)>
where
    V: Into<CommandArg>,
    T1: Into<CommandArg>,
    T2: Into<CommandArg>,
    T: IntoIterator<Item = (T1, T2)>,
    F: Into<CommandArg>,
{
    fn into_branch_opts(self) -> (Option<CommandArg>, CmdOpts, CommandArg) {
        let actions = self
            .0
             .1
            .into_iter()
            .flat_map(|(test, true_action)| [test.into().to_cmd(), true_action.into().to_cmd()])
            .collect();
        (
            Some(self.0 .0.into()),
            CmdOpts::Many(actions),
            self.0 .2.into(),
        )
    }
}
