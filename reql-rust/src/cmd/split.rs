use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde_json::Value;

use crate::Command;
use crate::ops::{ReqlOps, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct SplitBuilder(pub(crate) Command);

impl SplitBuilder {
    pub(crate) fn new(separator: Option<&str>, max_splits: Option<&str>) -> Self {
        let arg_separator = Command::from_json(separator);
        let mut command = Command::new(TermType::Split).with_arg(arg_separator);

        if let Some(max_splits) = max_splits {
            let arg_max_splits = Command::from_json(max_splits);
            command = command.with_arg(arg_max_splits);
        }

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Value>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Value>> {        
        self.get_parent().run::<_, Value>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOpsDocManipulation for SplitBuilder { }

impl ReqlOps for SplitBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for SplitBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
