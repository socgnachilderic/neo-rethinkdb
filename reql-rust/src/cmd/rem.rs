use std::ops::Rem;

use ql2::term::TermType;

use crate::Command;

impl<T: RemArg> Rem<T> for Command {
    type Output = Self;

    fn rem(self, arg: T) -> Self {
        Command::new(TermType::Mod)
            .with_arg(arg.into_rem_opts())
            .with_parent(self)
    }
}

pub trait RemArg {
    fn into_rem_opts(self) -> Command;
}

impl RemArg for f64 {
    fn into_rem_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl RemArg for Command {
    fn into_rem_opts(self) -> Command {
        self
    }
}

// TODO write test
