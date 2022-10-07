use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::arguments::Args;
use crate::prelude::Func;
use crate::Command;

pub(crate) fn new(args: impl EqJoinArg) -> Command {
    let (arg, right_table, opts) = args.into_eq_join_opts();

    Command::new(TermType::EqJoin)
        .with_arg(arg)
        .with_arg(right_table)
        .with_opts(opts)
}

pub trait EqJoinArg {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption);
}

impl<T> EqJoinArg for Args<(T, Command)>
where
    T: Into<String>,
{
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (
            Command::from_json(self.0 .0.into()),
            self.0 .1,
            Default::default(),
        )
    }
}

impl EqJoinArg for Args<(Func, Command)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0 .0;

        (func, self.0 .1, Default::default())
    }
}

impl EqJoinArg for Args<(Command, Command)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0 .0, self.0 .1, Default::default())
    }
}

impl<T> EqJoinArg for Args<(T, Command, EqJoinOption)>
where
    T: Into<String>,
{
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (Command::from_json(self.0 .0.into()), self.0 .1, self.0 .2)
    }
}

impl EqJoinArg for Args<(Func, Command, EqJoinOption)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0 .0;

        (func, self.0 .1, self.0 .2)
    }
}

impl EqJoinArg for Args<(Command, Command, EqJoinOption)> {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

#[derive(
    Debug, Clone, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Hash, CommandOptions,
)]
#[non_exhaustive]
pub struct EqJoinOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
}

// TODO write test
