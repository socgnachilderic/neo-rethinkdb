use ql2::term::TermType;

use crate::{Command, Geometry};

pub(crate) fn new(geometry: impl IntersectsArg) -> Command {
    let (arg1, arg) = geometry.into_intersects_opts();
    let mut command = Command::new(TermType::Intersects);

    if let Some(arg) = arg1 {
        command = command.with_arg(arg);
    }

    command.with_arg(arg)
}

pub trait IntersectsArg {
    fn into_intersects_opts(self) -> (Option<Command>, Command);
}

impl IntersectsArg for Command {
    fn into_intersects_opts(self) -> (Option<Command>, Command) {
        (None, self)
    }
}

impl<T> IntersectsArg for T
where
    T: Geometry,
{
    fn into_intersects_opts(self) -> (Option<Command>, Command) {
        (None, self.into())
    }
}
