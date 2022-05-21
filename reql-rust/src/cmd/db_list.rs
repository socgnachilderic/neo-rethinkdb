use std::borrow::Cow;

use futures::Stream;
use ql2::term::TermType;

use crate::Command;

use super::run;

pub struct DbListBuilder;

impl DbListBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Vec<Cow<'static, str>>>> {        
        Command::new(TermType::DbList)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<Cow<'static, str>>>(arg)
    }
}
