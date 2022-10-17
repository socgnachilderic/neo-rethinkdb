use ql2::term::TermType;

use crate::arguments::MaxOption;
use crate::{Command, Func};

pub(crate) fn new(args: impl MaxArg) -> Command {
    let (arg, opts) = args.into_max_opts();
    let mut command = Command::new(TermType::Max);

    if let Some(arg) = arg {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MaxArg {
    fn into_max_opts(self) -> (Option<Command>, MaxOption);
}

impl MaxArg for () {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (None, Default::default())
    }
}

impl MaxArg for &str {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        let arg = Command::from_json(self);

        (Some(arg), Default::default())
    }
}

impl MaxArg for String {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        let arg = Command::from_json(self);

        (Some(arg), Default::default())
    }
}

impl MaxArg for Func {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (Some(self.0), Default::default())
    }
}

impl MaxArg for MaxOption {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (None, self)
    }
}

impl MaxArg for Command {
    fn into_max_opts(self) -> (Option<Command>, MaxOption) {
        (Some(self), Default::default())
    }
}
