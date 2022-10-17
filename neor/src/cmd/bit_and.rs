use std::ops::BitAnd;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> BitAnd<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    fn bitand(self, number: T) -> Self {
        number
            .into()
            .add_to_cmd(TermType::BitAnd)
            .with_parent(&self)
    }
}
