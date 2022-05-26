use std::borrow::Cow;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;

use crate::Command;

#[derive(Debug, Clone)]
pub struct TableListBuilder(pub(crate) Command);

impl TableListBuilder {
    pub(crate) fn new() -> Self {
        TableListBuilder(Command::new(TermType::TableList))
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
