use std::collections::HashMap;

use ql2::term::TermType;

use crate::Command;

pub(crate) fn new<T>(args: HashMap<T, Command>) -> Command
where
    T: Into<String>,
{
    args.into_iter()
        .flat_map(|(key, value)| [Command::from_json(key.into()), value])
        .fold(Command::new(TermType::Object), |command, arg| {
            command.with_arg(arg)
        })
}
