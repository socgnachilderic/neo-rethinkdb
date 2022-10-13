use std::ops::Not;

use ql2::term::TermType;

use crate::Command;

impl Not for Command {
    type Output = Self;

    fn not(self) -> Self::Output {
        Command::new(TermType::Not).with_arg(self)
    }
}
