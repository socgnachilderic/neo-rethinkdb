use ql2::term::TermType;

use crate::arguments::{Args, IndexRenameOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl IndexRenameArg) -> Command {
    let (old_index_arg, new_index_arg, opts) = args.into_index_rename_opts();

    Command::new(TermType::IndexRename)
        .with_arg(old_index_arg.to_cmd())
        .with_arg(new_index_arg.to_cmd())
        .with_opts(opts)
}

pub trait IndexRenameArg {
    fn into_index_rename_opts(self) -> (CommandArg, CommandArg, IndexRenameOption);
}

impl<O, N> IndexRenameArg for Args<(O, N)>
where
    O: Into<CommandArg>,
    N: Into<CommandArg>,
{
    fn into_index_rename_opts(self) -> (CommandArg, CommandArg, IndexRenameOption) {
        (self.0 .0.into(), self.0 .1.into(), Default::default())
    }
}

impl<O, N> IndexRenameArg for Args<(O, N, IndexRenameOption)>
where
    O: Into<CommandArg>,
    N: Into<CommandArg>,
{
    fn into_index_rename_opts(self) -> (CommandArg, CommandArg, IndexRenameOption) {
        (self.0 .0.into(), self.0 .1.into(), self.0 .2)
    }
}
