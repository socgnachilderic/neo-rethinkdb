use std::ops::Div;

use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

impl<T: DivArg> Div<T> for Command {
    type Output = Self;

    fn div(self, arg: T) -> Self {
        Command::new(TermType::Div)
            .with_arg(arg.into_div_opts())
            .with_parent(self)
    }
}

pub trait DivArg {
    fn into_div_opts(self) -> Command;
}

impl<T: Serialize> DivArg for T {
    fn into_div_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl DivArg for Command {
    fn into_div_opts(self) -> Command {
        self
    }
}

// TODO write test
