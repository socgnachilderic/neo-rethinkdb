use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::TableDropReturnType};

use super::run;

pub struct TableDropBuilder(Command, Option<Command>);

impl TableDropBuilder {
    pub fn new(table_name: &str) -> Self {
        let args = Command::from_json(table_name);
        let command = Command::new(TermType::TableDrop)
            .with_arg(args);
        TableDropBuilder(command, None)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<TableDropReturnType>> {
        let mut cmd = self.0;

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, TableDropReturnType>(arg)
    }
}
