use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::Command;
use crate::ops::{ReqlOps, ReqlOpsArray};

#[derive(Debug, Clone)]
pub struct DbListBuilder(pub(crate) Command);

impl DbListBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::DbList);

        Self(command)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<Vec<Cow<'static, str>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = Result<Vec<Cow<'static, str>>, crate::ReqlError>> {
        self.get_parent().run::<_, Vec<Cow<'static, str>>>(arg)
    }
}

impl ReqlOpsArray for DbListBuilder {}

impl ReqlOps for DbListBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone().into_arg::<()>().into_cmd()
    }
}
