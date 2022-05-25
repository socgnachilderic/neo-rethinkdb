use std::borrow::Cow;

use futures::TryStreamExt;
use ql2::term::TermType;

use crate::Command;

use super::run;

pub struct DbListBuilder(Command);

impl DbListBuilder {
    pub fn new() -> Self {
        let command = Command::new(TermType::DbList);

        Self(command)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Vec<Cow<'static, str>>>> {        
        self.0.into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<Cow<'static, str>>>(arg)
            .try_next().await
    }
}
