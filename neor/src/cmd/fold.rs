use ql2::term::TermType;

use crate::{Command, CommandArg, Func};

pub(crate) fn new(base: impl Into<CommandArg>, func: Func) -> Command {
    base.into().add_to_cmd(TermType::Fold).with_arg(func.0)
}
