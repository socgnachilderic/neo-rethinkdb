use ql2::term::TermType;
use regex::Regex;

use crate::Command;

pub(crate) fn new(regex: Regex) -> Command {
    let arg = Command::from_json(regex.as_str());

    Command::new(TermType::Match).with_arg(arg)
}

// write test
