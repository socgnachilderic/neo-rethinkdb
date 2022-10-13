use ql2::term::TermType;

use crate::arguments::ChangesOption;
use crate::Command;

pub(crate) fn new(args: impl ChangesArg) -> Command {
    Command::new(TermType::Changes)
        .with_opts(args.into_changes_opts())
        .mark_change_feed()
}

pub trait ChangesArg {
    fn into_changes_opts(self) -> ChangesOption;
}

impl ChangesArg for () {
    fn into_changes_opts(self) -> ChangesOption {
        Default::default()
    }
}

impl ChangesArg for ChangesOption {
    fn into_changes_opts(self) -> ChangesOption {
        self
    }
}
