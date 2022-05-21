
use std::borrow::Cow;

use futures::Stream;
use ql2::term::TermType;

use crate::Command;

use super::run;

pub struct TableListBuilder(Option<Command>);

impl TableListBuilder {
    pub fn new() -> Self {
        TableListBuilder(None)
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = Some(parent);
        self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Vec<Cow<'static, str>>>> {        
        let mut cmd = Command::new(TermType::TableList);

        if let Some(parent) = self.0 {
            cmd = cmd.with_parent(parent);
        }
            
        let cmd = cmd.into_arg::<()>()
            .into_cmd();

        cmd.run::<_, Vec<Cow<'static, str>>>(arg)
    }
}

