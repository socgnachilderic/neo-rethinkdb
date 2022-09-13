use std::borrow::Cow;

use ql2::term::TermType;
use reql_macros::CommandOptions;
use serde::Serialize;

use crate::prelude::Func;
use crate::{types::AnyParam, Command};

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

impl EqJoinArg for (AnyParam, Command) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0.into(), self.1, Default::default())
    }
}

impl EqJoinArg for (Func, Command) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0;

        (func, self.1, Default::default())
    }
}

impl EqJoinArg for (Command, Command) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0, self.1, Default::default())
    }
}

impl EqJoinArg for (AnyParam, Command, EqJoinOption) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0.into(), self.1, self.2)
    }
}

impl EqJoinArg for (Func, Command, EqJoinOption) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        let Func(func) = self.0;

        (func, self.1, self.2)
    }
}

impl EqJoinArg for (Command, Command, EqJoinOption) {
    fn into_eq_join_opts(self) -> (Command, Command, EqJoinOption) {
        (self.0, self.1, self.2)
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
