use ql2::term::TermType;

use crate::arguments::GetIntersectingOption;
use crate::{Command, Geometry};

pub(crate) fn new(args: impl GetIntersectingArg, index: &'static str) -> Command {
    let opts = GetIntersectingOption::default().index(index);

    Command::new(TermType::GetIntersecting)
        .with_arg(args.into_get_intersecting_opts())
        .with_opts(opts)
}

pub trait GetIntersectingArg {
    fn into_get_intersecting_opts(self) -> Command;
}

impl GetIntersectingArg for Command {
    fn into_get_intersecting_opts(self) -> Command {
        self
    }
}

impl<T> GetIntersectingArg for T
where
    T: Geometry,
{
    fn into_get_intersecting_opts(self) -> Command {
        self.cmd()
    }
}
