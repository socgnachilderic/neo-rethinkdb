
use std::borrow::Cow;

use futures::TryStreamExt;
use ql2::term::TermType;

use crate::Command;

use super::run;

pub struct TableListBuilder(Command);

impl TableListBuilder {
    pub fn new() -> Self {
        TableListBuilder(Command::new(TermType::TableList))
    }

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Vec<Cow<'static, str>>>> {        
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<Cow<'static, str>>>(arg)
            .try_next().await
    }
}
