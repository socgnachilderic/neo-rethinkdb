use crate::{cmd, Command};
use ql2::term::TermType;
use std::ops::Rem;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}

impl Arg for Command {
    fn arg(self) -> cmd::Arg<()> {
        Self::new(TermType::Mod).with_arg(self).into_arg()
    }
}

impl<T> Rem<T> for Command
where
    T: Arg,
{
    type Output = Self;

    fn rem(self, arg: T) -> Self {
        arg.arg().with_parent(self).into_cmd()
    }
}
