use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::Command;
use crate::ops::{SuperOps, ReqlOpsSequence, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct DeleteAtBuilder(pub(crate) Command);

impl DeleteAtBuilder {
    pub(crate) fn new(offset: isize, end_offset: Option<isize>) -> Self {
        let arg_offset = Command::from_json(offset);
        let mut command = Command::new(TermType::DeleteAt)
            .with_arg(arg_offset);

        if let Some(end_offset) = end_offset {
            let arg_end_offset = Command::from_json(end_offset);
            command = command.with_arg(arg_end_offset);
        }

        Self(command)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Value>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Value>> {        
        self.0.into_arg::<()>().into_cmd().run::<_, Value>(arg)
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for DeleteAtBuilder { }

impl ReqlOpsDocManipulation for DeleteAtBuilder { }

impl SuperOps for DeleteAtBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
