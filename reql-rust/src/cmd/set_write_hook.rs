use futures::Stream;
use ql2::term::TermType;

use crate::{cmd, Func, Command};

pub struct SetWriteHookBuilder(Command);

impl SetWriteHookBuilder {
    pub fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::SetWriteHook).with_arg(func);

        Self(command)
    }

    pub fn run(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<serde_json::Value>> {       
        let cmd = self.0.into_arg::<()>().into_cmd();

        cmd.run::<_, serde_json::Value>(arg)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}
