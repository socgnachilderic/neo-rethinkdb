use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;
use serde_json::Value;

use crate::ops::{ReqlOpsArray, SuperOps, ReqlOpsDocManipulation};
use crate::Command;

#[derive(Debug, Clone)]
pub struct ValuesBuilder(pub(crate) Command);

impl ValuesBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Values);
        
        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Vec<Value>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<Vec<Value>>> {
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<Value>>(arg)
    }

    pub fn with_sequences(mut self, sequences: &[impl Serialize]) -> Self {
        for seq in sequences {
            let arg = Command::from_json(seq);
            self.0 = self.0.with_arg(arg)
        }

        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOpsArray for ValuesBuilder { }
impl ReqlOpsDocManipulation for ValuesBuilder { }

impl SuperOps for ValuesBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}
