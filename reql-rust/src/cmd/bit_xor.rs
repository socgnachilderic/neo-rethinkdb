use std::ops::BitXor;

use ql2::term::TermType;

use crate::Command;

impl<T: BitXorArg> BitXor<T> for Command {
    type Output = Self;

    fn bitxor(self, arg: T) -> Self {
        Command::new(TermType::BitXor)
            .with_arg(arg.into_bit_xor_opts())
            .with_parent(self)
    }
}

pub trait BitXorArg {
    fn into_bit_xor_opts(self) -> Command;
}

impl BitXorArg for f64 {
    fn into_bit_xor_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitXorArg for Vec<f64> {
    fn into_bit_xor_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl BitXorArg for Command {
    fn into_bit_xor_opts(self) -> Command {
        self
    }
}

// TODO write test
