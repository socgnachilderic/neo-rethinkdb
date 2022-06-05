use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::Command;
use crate::ops::{SuperOps, ReqlOpsSequence, ReqlOpsDocManipulation};

#[derive(Debug, Clone)]
pub struct ChangeAtBuilder(pub(crate) Command);

impl ChangeAtBuilder {
    pub(crate) fn new(offset: usize, value: impl Serialize) -> Self {
        let arg_offset = Command::from_json(offset);
        let arg_value = Command::from_json(value);
        let command = Command::new(TermType::ChangeAt)
            .with_arg(arg_offset)
            .with_arg(arg_value);

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

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for ChangeAtBuilder { }

impl ReqlOpsDocManipulation for ChangeAtBuilder { }

impl SuperOps for ChangeAtBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
