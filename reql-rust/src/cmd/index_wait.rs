use crate::Command;
use futures::Stream;
use ql2::term::TermType;

use super::run;

pub struct IndexWaitBuilder(Command, Option<Command>);

impl IndexWaitBuilder {
    pub fn new(index_name: &str) -> Self {
        let args = Command::from_json(index_name);
        let command = Command::new(TermType::IndexWait).with_arg(args);

        IndexWaitBuilder(command, None)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<serde_json::Value>> {
        let mut cmd = self.0;

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, serde_json::Value>(arg)
    }
}
