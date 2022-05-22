use futures::Stream;
use ql2::term::TermType;

use crate::{cmd, types::WriteHookResponseType, Command, Func};

pub struct SetWriteHookBuilder(Command);

impl SetWriteHookBuilder {
    pub fn new(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::SetWriteHook).with_arg(func);

        Self(command)
    }

    pub fn run(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<WriteHookResponseType>> {
        let cmd = self.0.into_arg::<()>().into_cmd();

        cmd.run::<_, WriteHookResponseType>(arg)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}
