use ql2::term::TermType;

use crate::{arguments::DistinctOption, Command};

pub(crate) fn new(args: impl DistinctArg) -> Command {
    let (args, opts) = args.into_distinct_opts();
    let mut command = Command::new(TermType::Distinct);

    if let Some(arg) = args {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait DistinctArg {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption);
}

impl DistinctArg for () {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (None, Default::default())
    }
}

impl DistinctArg for DistinctOption {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (None, self)
    }
}

impl DistinctArg for Command {
    fn into_distinct_opts(self) -> (Option<Command>, DistinctOption) {
        (Some(self), Default::default())
    }
}
