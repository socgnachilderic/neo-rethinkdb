use std::borrow::Cow;

use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::DbCreateReturnType};

use super::{StaticString, run};

pub struct DbCreateBuilder(Cow<'static, str>);

impl DbCreateBuilder {
    pub fn new(db_name: &'static str) -> Self {
        DbCreateBuilder(db_name.static_string())
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<DbCreateReturnType>> {
        let args = Command::from_json(&self.0);
        
        Command::new(TermType::DbCreate)
            .with_arg(args)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbCreateReturnType>(arg)
    }
}
