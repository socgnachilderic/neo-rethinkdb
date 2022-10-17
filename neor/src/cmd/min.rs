use ql2::term::TermType;

use crate::arguments::MinOption;
use crate::{Command, Func};

pub(crate) fn new(args: impl MinArg) -> Command {
    let (arg, opts) = args.into_min_opts();
    let mut command = Command::new(TermType::Min);

    if let Some(arg) = arg {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait MinArg {
    fn into_min_opts(self) -> (Option<Command>, MinOption);
}

impl MinArg for () {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (None, Default::default())
    }
}

impl MinArg for &str {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        let arg = Command::from_json(self);

        (Some(arg), Default::default())
    }
}

impl MinArg for String {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        let arg = Command::from_json(self);

        (Some(arg), Default::default())
    }
}

impl MinArg for Func {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (Some(self.0), Default::default())
    }
}

impl MinArg for MinOption {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (None, self)
    }
}

impl MinArg for Command {
    fn into_min_opts(self) -> (Option<Command>, MinOption) {
        (Some(self), Default::default())
    }
}
