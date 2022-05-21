use std::borrow::Cow;

use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::TableDropReturnType};

use super::{StaticString, run};

pub struct TableDropBuilder(Cow<'static, str>, Option<Command>);

impl TableDropBuilder {
    pub fn new(table_name: &'static str) -> Self {
        TableDropBuilder(table_name.static_string(), None)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.1 = Some(parent);
        self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<TableDropReturnType>> {
        let args = Command::from_json(&self.0);
        let mut cmd = Command::new(TermType::TableCreate)
            .with_arg(args);

        if let Some(parent) = self.1 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, TableDropReturnType>(arg)
    }
}

impl Into<Command> for TableDropBuilder {
    fn into(self) -> Command {
        Command::from_json(self.0)
    }
}
