use ql2::term::TermType;

use crate::arguments::{Args, DistanceOption};
use crate::{Command, Geometry};

pub(crate) fn new(args: impl DistanceArg) -> Command {
    let (arg1, arg2, opts) = args.into_distance_opts();
    let mut command = Command::new(TermType::Distance).with_arg(arg1);

    if let Some(arg) = arg2 {
        command = command.with_arg(arg)
    }

    command.with_opts(opts)
}

pub trait DistanceArg {
    fn into_distance_opts(self) -> (Command, Option<Command>, DistanceOption);
}

impl<T: Geometry> DistanceArg for T {
    fn into_distance_opts(self) -> (Command, Option<Command>, DistanceOption) {
        (self.into(), None, Default::default())
    }
}

impl<T: Geometry> DistanceArg for Args<(T, DistanceOption)> {
    fn into_distance_opts(self) -> (Command, Option<Command>, DistanceOption) {
        (self.0 .0.into(), None, self.0 .1)
    }
}
