use ql2::term::TermType;
use time::UtcOffset;

use crate::types::timezone_to_string;
use crate::Command;

pub(crate) fn new(timezone: UtcOffset) -> Command {
    Command::new(TermType::InTimezone).with_arg(Command::from_json(timezone_to_string(timezone)))
}
