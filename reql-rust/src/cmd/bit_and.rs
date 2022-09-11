use std::ops::BitAnd;

use ql2::term::TermType;

use crate::Command;

impl<T: BitAndArg> BitAnd<T> for Command {
    type Output = Self;

    fn bitand(self, arg: T) -> Self {
        Command::new(TermType::BitAnd)
            .with_arg(arg.into_bit_and_opts())
            .with_parent(self)
    }
}

pub trait BitAndArg {
    fn into_bit_and_opts(self) -> Command;
}

impl BitAndArg for f64 {
    fn into_bit_and_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitAndArg for Vec<f64> {
    fn into_bit_and_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitAndArg for Command {
    fn into_bit_and_opts(self) -> Command {
        self
    }
}

// TODO write test
