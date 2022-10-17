use std::collections::HashMap;

use ql2::term::TermType;

use crate::{Command, CommandArg};

pub(crate) fn new<K, V>(args: HashMap<K, V>) -> Command
where
    K: Into<CommandArg>,
    V: Into<CommandArg>,
{
    args.into_iter()
        .flat_map(|(key, value)| [key.into().to_cmd(), value.into().to_cmd()])
        .fold(Command::new(TermType::Object), |command, arg| {
            command.with_arg(arg)
        })
}
