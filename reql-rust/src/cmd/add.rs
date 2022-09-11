use std::ops::Add;

use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

impl<T: AddArg> Add<T> for Command {
    type Output = Self;

    fn add(self, arg: T) -> Self {
        Command::new(TermType::Add)
            .with_arg(arg.into_add_opts())
            .with_parent(self)
    }
}

pub trait AddArg {
    fn into_add_opts(self) -> Command;
}

impl<T: Serialize> AddArg for T {
    fn into_add_opts(self) -> Command {
        Command::from_json(self)
    }
}

impl AddArg for Command {
    fn into_add_opts(self) -> Command {
        self
    }
}

// TODO write test
