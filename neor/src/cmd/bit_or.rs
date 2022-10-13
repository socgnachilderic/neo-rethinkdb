use std::ops::BitOr;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> BitOr<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    fn bitor(self, number: T) -> Self {
        number.into().add_to_cmd(TermType::BitOr).with_parent(&self)
    }
}
