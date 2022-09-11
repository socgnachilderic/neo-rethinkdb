use std::ops::BitOr;

use ql2::term::TermType;

use crate::Command;

impl<T: BitOrArg> BitOr<T> for Command {
    type Output = Self;

    fn bitor(self, arg: T) -> Self {
        Command::new(TermType::BitOr)
            .with_arg(arg.into_bit_or_opts())
            .with_parent(self)
    }
}

pub trait BitOrArg {
    fn into_bit_or_opts(self) -> Command;
}

impl BitOrArg for f64 {
    fn into_bit_or_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitOrArg for Vec<f64> {
    fn into_bit_or_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitOrArg for Command {
    fn into_bit_or_opts(self) -> Command {
        self
    }
}

// TODO write test
