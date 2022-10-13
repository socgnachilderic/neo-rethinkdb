use ql2::term::TermType;

use crate::arguments::{Args, IndexCreateOption};
use crate::{Command, CommandArg, Func};

pub(crate) fn new(args: impl IndexCreateArg) -> Command {
    let (arg, func, opts) = args.into_table_create_opts();
    let mut command = arg.add_to_cmd(TermType::IndexCreate);

    if let Some(Func(func)) = func {
        command = command.with_arg(func);
    }

    command.with_opts(opts)
}

pub trait IndexCreateArg {
    fn into_table_create_opts(self) -> (CommandArg, Option<Func>, IndexCreateOption);
}

impl<T> IndexCreateArg for T
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, Option<Func>, IndexCreateOption) {
        (self.into(), None, Default::default())
    }
}

impl<T> IndexCreateArg for Args<(T, Func)>
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, Option<Func>, IndexCreateOption) {
        (self.0 .0.into(), Some(self.0 .1), Default::default())
    }
}

impl<T> IndexCreateArg for Args<(T, IndexCreateOption)>
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, Option<Func>, IndexCreateOption) {
        (self.0 .0.into(), None, self.0 .1)
    }
}

impl<T> IndexCreateArg for Args<(T, Func, IndexCreateOption)>
where
    T: Into<CommandArg>,
{
    fn into_table_create_opts(self) -> (CommandArg, Option<Func>, IndexCreateOption) {
        (self.0 .0.into(), Some(self.0 .1), self.0 .2)
    }
}
