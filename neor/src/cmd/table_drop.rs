use ql2::term::TermType;

use crate::{command_tools::CommandArg, Command};

pub(crate) fn new(table_name: impl Into<CommandArg>) -> Command {
    table_name.into().add_to_cmd(TermType::TableDrop)
}
