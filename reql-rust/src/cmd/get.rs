use std::marker::PhantomData;

use crate::{Command, document::Document, ops::{ReqlOpsSequence, SuperOps}};
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct GetBuilder<T>(pub(crate) Command, pub(crate) PhantomData<T>);

impl<T: Unpin + DeserializeOwned> GetBuilder<T> {
    pub(crate) fn new(primary_key: impl Serialize) -> Self {
        let args = Command::from_json(primary_key);
        let command = Command::new(TermType::Get).with_arg(args);

        Self(command, PhantomData)
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<Option<Document<T>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl super::run::Arg) -> impl Stream<Item = crate::Result<Option<Document<T>>>> {
        self.0.into_arg::<()>().into_cmd().run::<_, Option<Document<T>>>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> ReqlOpsSequence<T> for GetBuilder<T> { }

impl<T> SuperOps for GetBuilder<T> {
    fn get_parent(&self) -> Command {
        self.0.clone()
    }
}

impl<T> Into<Command> for GetBuilder<T> {
    fn into(self) -> Command {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd, r};

    #[test]
    fn r_db_table_get() {
        let query = r
            .db("foo")
            .table::<serde_json::Value>("bar")
            .get("84fc23ac-9e85-43af-b6f7-f86be17237e1");
        let serialised = cmd::serialise(&query.into());
        let expected = r#"[16,[[15,[[14,["foo"]],"bar"]],"84fc23ac-9e85-43af-b6f7-f86be17237e1"]]"#;
        assert_eq!(serialised, expected);
    }
}
