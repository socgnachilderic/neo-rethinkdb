use ql2::term::TermType;

use crate::arguments::DeleteOption;
use crate::Command;

pub(crate) fn new(args: impl DeleteArg) -> Command {
    Command::new(TermType::Delete).with_opts(args.into_delete_opts())
}

pub trait DeleteArg {
    fn into_delete_opts(self) -> DeleteOption;
}

impl DeleteArg for () {
    fn into_delete_opts(self) -> DeleteOption {
        Default::default()
    }
}
impl DeleteArg for DeleteOption {
    fn into_delete_opts(self) -> DeleteOption {
        self
    }
}
