use futures::Stream;
use ql2::term::TermType;
use serde::Serialize;

use crate::Command;
use crate::types::IndexResponseType;

use super::run;

pub struct IndexRenameBuilder(Command, IndexRenameOption, Option<Command>);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct IndexRenameOption {
    pub overwrite: Option<bool>,
}

impl IndexRenameBuilder {
    pub fn new(old_index_name: &str, new_index_name: &str) -> Self {
        let arg_1 = Command::from_json(old_index_name);
        let arg_2 = Command::from_json(new_index_name);
        let command = Command::new(TermType::IndexRename).with_arg(arg_1).with_arg(arg_2);
        
        IndexRenameBuilder(command, Default::default(), None)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<IndexResponseType>> {        
        let mut cmd = self.0.with_opts(self.1);

        if let Some(parent) = self.2 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, IndexResponseType>(arg)
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.1.overwrite = Some(overwrite);
        self
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.2 = Some(parent);
        self
    }
}
