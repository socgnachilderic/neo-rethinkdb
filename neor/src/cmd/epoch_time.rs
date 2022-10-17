use ql2::term::TermType;

use crate::Command;

pub(crate) fn epoch_time(timestamp: i64) -> Command {
    Command::new(TermType::EpochTime).with_arg(Command::from_json(timestamp))
}
