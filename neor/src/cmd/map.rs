use ql2::term::TermType;

use crate::arguments::Args;
use crate::command_tools::CmdOpts;
use crate::{Command, Func};

pub(crate) fn new(args: impl MapArg) -> Command {
    let (args, func) = args.into_map_opts();
    let mut command = Command::new(TermType::Map);

    if let Some(args) = args {
        command = args.add_to_cmd(command);
    }

    command.with_arg(func)
}

pub trait MapArg {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command);
}

impl MapArg for Func {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        (None, self.0)
    }
}

impl MapArg for Args<(Command, Func)> {
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        let Func(func) = self.0 .1;

        (Some(CmdOpts::Single(self.0 .0)), func)
    }
}

impl<T> MapArg for Args<(T, Func)>
where
    T: IntoIterator<Item = Command>,
{
    fn into_map_opts(self) -> (Option<CmdOpts>, Command) {
        let Func(func) = self.0 .1;

        (Some(CmdOpts::Many(self.0 .0.into_iter().collect())), func)
    }
}
