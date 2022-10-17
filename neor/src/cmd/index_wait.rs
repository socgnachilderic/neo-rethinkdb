use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl IndexWaitArg) -> Command {
    let mut command = Command::new(TermType::IndexWait);

    if let Some(args) = args.into_index_wait_opts() {
        command = args.add_to_cmd(command)
    }

    command
}

pub trait IndexWaitArg {
    fn into_index_wait_opts(self) -> Option<CmdOpts>;
}

impl IndexWaitArg for () {
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        None
    }
}

impl IndexWaitArg for &str {
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        let arg = Command::from_json(self);

        Some(CmdOpts::Single(arg))
    }
}

impl<C, T> IndexWaitArg for Args<T>
where
    C: Into<CommandArg>,
    T: IntoIterator<Item = C>,
{
    fn into_index_wait_opts(self) -> Option<CmdOpts> {
        Some(CmdOpts::Many(
            self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect(),
        ))
    }
}
