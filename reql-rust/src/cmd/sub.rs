use std::ops::Sub;

use ql2::term::TermType;

use crate::Command;

impl<T: SubArg> Sub<T> for Command {
    type Output = Self;

    fn sub(self, arg: T) -> Self {
        Command::new(TermType::Sub)
            .with_arg(arg.into_sub_opts())
            .with_parent(self)
    }
}

pub trait SubArg {
    fn into_sub_opts(self) -> Command;
}

impl SubArg for f64 {
    fn into_sub_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SubArg for Vec<f64> {
    fn into_sub_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl SubArg for Command {
    fn into_sub_opts(self) -> Command {
        self
    }
}

// TODO write test
