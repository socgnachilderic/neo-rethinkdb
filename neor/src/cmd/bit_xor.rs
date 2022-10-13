use std::ops::BitXor;

use ql2::term::TermType;

use crate::{Command, CommandArg};

impl<T> BitXor<T> for Command
where
    T: Into<CommandArg>,
{
    type Output = Self;

    fn bitxor(self, number: T) -> Self {
        number
            .into()
            .add_to_cmd(TermType::BitXor)
            .with_parent(&self)
    }
}
