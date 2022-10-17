use ql2::term::TermType;
use serde::Serialize;

use crate::arguments::Args;
use crate::Command;

pub(crate) fn new<T>(args: impl HttpArg<T>) -> Command
where
    T: Serialize,
{
    let (arg, opts) = args.into_http_opts();
    let mut command = Command::new(TermType::Http).with_arg(arg);

    if let Some(opts) = opts {
        command = command.with_opts(opts);
    }

    command
}

pub trait HttpArg<T: Serialize> {
    fn into_http_opts(self) -> (Command, Option<T>);
}

impl<T> HttpArg<T> for T
where
    T: Into<String> + Serialize,
{
    fn into_http_opts(self) -> (Command, Option<T>) {
        (Command::from_json(self.into()), None)
    }
}

impl<S, T> HttpArg<T> for Args<(S, T)>
where
    S: Into<String>,
    T: Serialize,
{
    fn into_http_opts(self) -> (Command, Option<T>) {
        (Command::from_json(self.0 .0.into()), Some(self.0 .1))
    }
}
