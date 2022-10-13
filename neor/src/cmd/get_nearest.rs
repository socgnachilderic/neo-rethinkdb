use ql2::term::TermType;

use crate::arguments::{Args, GetNearestOption};
use crate::{Command, Geometry};

pub(crate) fn new(args: impl GetNearestArg) -> Command {
    let (arg, opts) = args.into_get_nearest_opts();

    Command::new(TermType::GetNearest)
        .with_arg(arg)
        .with_opts(opts)
}

pub trait GetNearestArg {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption);
}

impl<T: Geometry> GetNearestArg for Args<(T, &str)> {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = GetNearestOption::default().index(self.0 .1.to_owned());

        (self.0 .0.into(), index_name)
    }
}

impl GetNearestArg for Args<(Command, &str)> {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = GetNearestOption::default().index(self.0 .1.to_owned());

        (self.0 .0, index_name)
    }
}

impl<T: Geometry> GetNearestArg for Args<(T, &str, GetNearestOption)> {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = self.0 .2.index(self.0 .1.to_owned());

        (self.0 .0.into(), index_name)
    }
}

impl GetNearestArg for Args<(Command, &str, GetNearestOption)> {
    fn into_get_nearest_opts(self) -> (Command, GetNearestOption) {
        let index_name = self.0 .2.index(self.0 .1.to_owned());

        (self.0 .0, index_name)
    }
}
