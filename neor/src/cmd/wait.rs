use ql2::term::TermType;

use crate::arguments::{Args, WaitOption};
use crate::Command;

pub(crate) fn new(args: impl WaitArg) -> Command {
    let (args, opts) = args.into_wait_opts();
    let mut command = Command::new(TermType::Wait);

    if let Some(arg) = args {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait WaitArg {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption);
}

impl WaitArg for () {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        Default::default()
    }
}

impl WaitArg for WaitOption {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (None, self)
    }
}

impl WaitArg for Command {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (Some(self), Default::default())
    }
}

impl WaitArg for Args<(Command, WaitOption)> {
    fn into_wait_opts(self) -> (Option<Command>, WaitOption) {
        (Some(self.0 .0), self.0 .1)
    }
}
