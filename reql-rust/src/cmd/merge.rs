use ql2::term::TermType;

use crate::types::AnyParam;
use crate::{Command, Func};

use super::CmdOpts;

pub(crate) fn new(args: impl MergeArg) -> Command {
    args.into_merge_opts()
        .add_to_cmd(Command::new(TermType::Merge))
}

pub trait MergeArg {
    fn into_merge_opts(self) -> CmdOpts;
}

impl MergeArg for AnyParam {
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Single(self.into())
    }
}

impl MergeArg for Command {
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Single(self)
    }
}

impl MergeArg for Func {
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Single(self.0)
    }
}

impl MergeArg for Vec<Command> {
    fn into_merge_opts(self) -> CmdOpts {
        CmdOpts::Many(self)
    }
}

impl MergeArg for Vec<Func> {
    fn into_merge_opts(self) -> CmdOpts {
        let cmds = self.into_iter().map(|func| func.0).collect();

        CmdOpts::Many(cmds)
    }
}

// TODO write test
