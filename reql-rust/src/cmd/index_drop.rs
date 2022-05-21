use crate::Command;
use crate::types::IndexResponseType;
use futures::Stream;
use ql2::term::TermType;

use super::run;

pub struct IndexDropBuilder(Command, Option<Command>);

impl IndexDropBuilder {
    pub fn new(index_name: &str) -> Self {
        let args = Command::from_json(index_name);
        let command = Command::new(TermType::IndexDrop).with_arg(args);

        IndexDropBuilder(command, None)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<IndexResponseType>> {
        let mut cmd = self.0;

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, IndexResponseType>(arg)
    }
}
