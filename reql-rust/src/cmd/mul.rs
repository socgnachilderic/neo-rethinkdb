use std::ops::Mul;

use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

impl<T: MulArg> Mul<T> for Command {
    type Output = Self;

    fn mul(self, arg: T) -> Self {
        Command::new(TermType::Mul)
            .with_arg(arg.into_mul_opts())
            .with_parent(self)
    }
}

pub trait MulArg {
    fn into_mul_opts(self) -> Command;
}

impl<T: AsRef<[f64]> + Serialize> MulArg for T {
    fn into_mul_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl MulArg for Command {
    fn into_mul_opts(self) -> Command {
        self
    }
}

// TODO write test
