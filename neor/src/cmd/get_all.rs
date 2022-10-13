use ql2::term::TermType;

use crate::arguments::{Args, GetAllOption};
use crate::command_tools::CmdOpts;
use crate::Command;

pub(crate) fn new(args: impl GetAllArg) -> Command {
    let (args, opts) = args.into_get_all_opts();

    args.add_to_cmd(Command::new(TermType::GetAll))
        .with_opts(opts)
}

pub trait GetAllArg {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption);
}

impl<S, T> GetAllArg for T
where
    S: Into<String>,
    T: IntoIterator<Item = S>,
{
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        let keys = self
            .into_iter()
            .map(|key| Command::from_json(key.into()))
            .collect();

        (CmdOpts::Many(keys), Default::default())
    }
}

impl GetAllArg for Command {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        (CmdOpts::Single(self), Default::default())
    }
}

impl<S, T> GetAllArg for Args<(T, GetAllOption)>
where
    S: Into<String>,
    T: IntoIterator<Item = S>,
{
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        let keys = self
            .0
             .0
            .into_iter()
            .map(|key| Command::from_json(key.into()))
            .collect();

        (CmdOpts::Many(keys), self.0 .1)
    }
}

impl GetAllArg for Args<(Command, GetAllOption)> {
    fn into_get_all_opts(self) -> (CmdOpts, GetAllOption) {
        (CmdOpts::Single(self.0 .0), self.0 .1)
    }
}
