use ql2::term::TermType;
use serde::Serialize;

use crate::{Command, Func};

pub enum CmdOpts {
    Single(Command),
    Many(Vec<Command>),
}

impl CmdOpts {
    pub(crate) fn add_to_cmd(self, command: Command) -> Command {
        match self {
            Self::Single(arg) => command.with_arg(arg),
            Self::Many(args) => args.into_iter().fold(command, |cmd, arg| cmd.with_arg(arg)),
        }
    }
}

impl From<CmdOpts> for Option<Command> {
    fn from(command: CmdOpts) -> Self {
        if let CmdOpts::Single(arg) = command {
            Some(arg)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandArg(Command);

impl CommandArg {
    pub fn add_to_cmd(self, typ: TermType) -> Command {
        Command::new(typ).with_arg(self.to_cmd())
    }

    pub fn to_cmd(self) -> Command {
        self.0
    }
}

impl<T: Serialize> From<T> for CommandArg {
    fn from(arg: T) -> Self {
        Command::from_json(arg).into()
    }
}

impl From<Command> for CommandArg {
    fn from(arg: Command) -> Self {
        CommandArg(arg)
    }
}

impl From<&Command> for CommandArg {
    fn from(arg: &Command) -> Self {
        CommandArg(arg.clone())
    }
}

impl From<Func> for CommandArg {
    fn from(arg: Func) -> Self {
        CommandArg(arg.0)
    }
}
