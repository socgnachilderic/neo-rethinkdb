use std::borrow::Cow;

use futures::Stream;
use ql2::term::TermType;

use crate::{Command, types::DbDropReturnType};

use super::{StaticString, run};

pub struct DbDropBuilder(Cow<'static, str>);

impl DbDropBuilder {
    pub fn new(db_name: &'static str) -> Self {
       DbDropBuilder(db_name.static_string())
    }

    pub fn run(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<DbDropReturnType>> {
        let args = Command::from_json(&self.0);
        
        Command::new(TermType::DbDrop)
            .with_arg(args)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, DbDropReturnType>(arg)
    }
}

