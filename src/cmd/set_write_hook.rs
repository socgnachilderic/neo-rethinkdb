use ql2::term::TermType;

use crate::prelude::Func;
use crate::{types::Binary, Command};

use super::CmdOpts;

pub(crate) fn new(args: Option<impl SetWriteHookArg>) -> Command {
    let mut command = Command::new(TermType::SetWriteHook);

    if let Some(arg) = args {
        command = arg.into_set_write_hook_opts().add_to_cmd(command);
    }

    command
}

pub trait SetWriteHookArg {
    fn into_set_write_hook_opts(self) -> CmdOpts;
}

impl SetWriteHookArg for Func {
    fn into_set_write_hook_opts(self) -> CmdOpts {
        CmdOpts::Single(self.0)
    }
}

impl SetWriteHookArg for Binary {
    fn into_set_write_hook_opts(self) -> CmdOpts {
        let arg = Command::from_json(self);

        CmdOpts::Single(arg)
    }
}

// WriteHookResponse
// TODO Write test
