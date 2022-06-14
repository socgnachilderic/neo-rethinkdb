use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::{Command, ops::ReqlOpsArray};

use super::{run, ReqlOps};

#[derive(Debug, Clone)]
pub struct IndexListBuilder(pub(crate) Command);

impl IndexListBuilder {
    pub(crate) fn new() -> Self {
        IndexListBuilder(Command::new(TermType::IndexList))
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Vec<Cow<'static, str>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl run::Arg,
    ) -> impl Stream<Item = crate::Result<Vec<Cow<'static, str>>>> {
        self.0
            .into_arg::<()>()
            .into_cmd()
            .run::<_, Vec<Cow<'static, str>>>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOpsArray for IndexListBuilder { }

impl ReqlOps for IndexListBuilder {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}

