use super::args::Args;
use crate::cmd::{self, Durability, ReturnChanges};
use crate::{Command, Func};
use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

// TODO finish this struct
#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<Options>;
}

impl Arg for cmd::Arg<Options> {
    fn arg(self) -> cmd::Arg<Options> {
        self
    }
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<Options> {
        Command::new(TermType::Replace).with_arg(self).into_arg()
    }
}

impl<T> Arg for T
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        Command::from_json(self).arg()
    }
}

impl Arg for Args<(Command, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((arg, opts)) = self;
        arg.arg().with_opts(opts)
    }
}

impl<T> Arg for Args<(T, Options)>
where
    T: Serialize,
{
    fn arg(self) -> cmd::Arg<Options> {
        let Args((arg, opts)) = self;
        let arg = Command::from_json(arg);
        arg.arg().with_opts(opts)
    }
}

impl Arg for Func {
    fn arg(self) -> cmd::Arg<Options> {
        let Func(func) = self;
        func.arg()
    }
}

impl Arg for Args<(Func, Options)> {
    fn arg(self) -> cmd::Arg<Options> {
        let Args((Func(func), opts)) = self;
        func.arg().with_opts(opts)
    }
}
