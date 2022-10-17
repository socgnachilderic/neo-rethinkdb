use ql2::term::TermType;

use crate::{command_tools::CmdOpts, Command, CommandArg};

pub(crate) fn new(expr: impl Into<CommandArg>) -> Command {
    expr.into().add_to_cmd(TermType::Funcall)
}

pub trait DoArg {
    fn into_do_opts(self) -> CmdOpts;
}

impl DoArg for Command {
    fn into_do_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl<S, T> DoArg for T
where
    S: Into<CommandArg>,
    T: IntoIterator<Item = S>,
{
    fn into_do_opts(self) -> CmdOpts {
        CmdOpts::Many(self.into_iter().map(|cmd| cmd.into().to_cmd()).collect())
    }
}
