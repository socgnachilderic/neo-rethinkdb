use ql2::term::TermType;

use crate::arguments::ReconfigureOption;
use crate::Command;

pub(crate) fn new(opts: ReconfigureOption) -> Command {
    Command::new(TermType::Reconfigure).with_opts(opts)
}
