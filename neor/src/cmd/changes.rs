use ql2::term::TermType;

use crate::arguments::ChangesOption;
use crate::Command;

pub(crate) fn new(args: impl ChangesArg) -> Command {
    let mut command = Command::new(TermType::Changes);

    if let Some(opts) = args.into_changes_opts() {
        command = command.with_opts(opts)
    }

    command.mark_change_feed()
}

pub trait ChangesArg {
    fn into_changes_opts(self) -> Option<ChangesOption>;
}

impl ChangesArg for () {
    fn into_changes_opts(self) -> Option<ChangesOption> {
        None
    }
}

impl ChangesArg for ChangesOption {
    fn into_changes_opts(self) -> Option<ChangesOption> {
        Some(self)
    }
}
