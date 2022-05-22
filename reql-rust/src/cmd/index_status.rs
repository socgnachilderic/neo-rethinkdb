use crate::Command;
use crate::types::IndexStatusResponseType;
use futures::Stream;
use ql2::term::TermType;

use super::run;

pub struct IndexStatusBuilder(Command, Option<Command>);

impl IndexStatusBuilder {
    /// Get all indexes of table
    pub fn new() -> Self {
        let command = Command::new(TermType::IndexStatus);

        IndexStatusBuilder(command, None)
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Vec<IndexStatusResponseType>>> {
        let mut cmd = self.0;

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, Vec<IndexStatusResponseType>>(arg)
    }

    /// Get one index of table
    pub fn with_one_index(mut self, index_name: &str) -> Self {
        let args = Command::from_json(index_name);

        self.0 = self.0.with_arg(args);
        self
    }


    /// Get an index array of table
    pub fn with_indexes(mut self, index_names: &[&str]) -> Self {
        for index_name in index_names {
            let args = Command::from_json(index_name);
            self.0 = self.0.with_arg(args);
        }
        
        self
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }
}
