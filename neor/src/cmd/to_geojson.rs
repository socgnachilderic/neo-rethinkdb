use ql2::term::TermType;

use crate::Command;

pub(crate) fn new() -> Command {
    Command::new(TermType::ToGeojson)
}

#[cfg(test)]
mod tests {}
