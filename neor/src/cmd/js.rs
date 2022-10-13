use ql2::term::TermType;

use crate::arguments::{Args, JsOption};
use crate::{Command, CommandArg};

pub(crate) fn new(args: impl JsArg) -> Command {
    let (arg, opts) = args.into_js_opts();

    arg.add_to_cmd(TermType::Javascript).with_opts(opts)
}

pub trait JsArg {
    fn into_js_opts(self) -> (CommandArg, JsOption);
}

impl<T> JsArg for T
where
    T: Into<CommandArg>,
{
    fn into_js_opts(self) -> (CommandArg, JsOption) {
        (self.into(), Default::default())
    }
}

impl<T> JsArg for Args<(T, JsOption)>
where
    T: Into<CommandArg>,
{
    fn into_js_opts(self) -> (CommandArg, JsOption) {
        (self.0 .0.into(), self.0 .1)
    }
}
