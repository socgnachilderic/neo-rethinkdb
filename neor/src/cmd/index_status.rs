use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl IndexStatusArg) -> Command {
    let mut command = Command::new(TermType::IndexStatus);
    let args = args.into_index_status_opts();

    if let Some(args) = args {
        command = args.add_to_cmd(command)
    }

    command
}

pub trait IndexStatusArg {
    fn into_index_status_opts(self) -> Option<CmdOpts>;
}

impl IndexStatusArg for () {
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        None
    }
}

impl IndexStatusArg for &str {
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        let arg = Command::from_json(self);

        Some(CmdOpts::Single(arg))
    }
}

impl<C, T> IndexStatusArg for Args<T>
where
    C: Into<CommandArg>,
    T: IntoIterator<Item = C>,
{
    fn into_index_status_opts(self) -> Option<CmdOpts> {
        Some(CmdOpts::Many(
            self.0.into_iter().map(|cmd| cmd.into().to_cmd()).collect(),
        ))
    }
}
