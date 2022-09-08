use ql2::term::TermType;

use crate::{Command, Func, types::Binary};

pub(crate) fn new(args: Option<impl SetWriteHookArg>) -> Command {
    let mut command = Command::new(TermType::SetWriteHook);
    
    if let Some(arg) = args {
        command = command.with_arg(arg.into_set_write_hook_opts());
    }
    
    command
}

pub trait SetWriteHookArg {
    fn into_set_write_hook_opts(self) -> Command;
}

impl SetWriteHookArg for Func {
    fn into_set_write_hook_opts(self) -> Command {
        self.0
    }
}

impl SetWriteHookArg for Binary {
    fn into_set_write_hook_opts(self) -> Command {
        Command::from_json(self)
    }
}

// WriteHookResponse
// TODO Write test
