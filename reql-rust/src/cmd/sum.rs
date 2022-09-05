use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::ops::ReqlOps;
use crate::{Command, Func};

#[derive(Debug, Clone)]
pub struct SumBuilder(pub(crate) Command);

impl SumBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Sum);
        Self(command)
    }

    pub(crate) fn new_by_field(field_name: &str) -> Self {
        let arg = Command::from_json(field_name);
        let command = Command::new(TermType::Sum).with_arg(arg);
        Self(command)
    }

    pub(crate) fn new_by_func(func: Func) -> Self {
        let Func(func) = func;
        let command = Command::new(TermType::Sum).with_arg(func);
        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<f64>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<f64>> {
        self.get_parent().run::<_, f64>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for SumBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for SumBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
