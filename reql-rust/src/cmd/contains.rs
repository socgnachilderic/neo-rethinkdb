use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::{Command, Func};
use crate::ops::ReqlOps;

#[derive(Debug, Clone)]
pub struct ContainsBuilder(pub(crate) Command);

impl ContainsBuilder {
    pub(crate) fn new(values: impl Serialize) -> Self {
        let arg = Command::from_json(values);
        let command = Command::new(TermType::Contains).with_arg(arg);
        Self(command)
    }

    pub(crate) fn new_by_func(funcs: Vec<Func>) -> Self {
        let mut command = Command::new(TermType::Contains);

        for func in funcs.into_iter() {
            let Func(func) = func;
            command = command.with_arg(func);
        }
        
        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<bool>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<bool>> {        
        self.0.into_arg::<()>().into_cmd().run::<_, bool>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for ContainsBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
