use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::ops::{ReqlOps, ReqlOpsDocManipulation, ReqlOpsSequence};
use crate::Command;

#[derive(Debug, Clone)]
pub struct HasFieldsBuilder(pub(crate) Command);

impl HasFieldsBuilder {
    pub(crate) fn new(fields: impl Serialize) -> Self {
        let arg = Command::from_json(fields);
        let command = Command::new(TermType::HasFields).with_arg(arg);

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

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for HasFieldsBuilder {}

impl ReqlOpsDocManipulation for HasFieldsBuilder {}

impl ReqlOps for HasFieldsBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}

impl Into<Command> for HasFieldsBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
