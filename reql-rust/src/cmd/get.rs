use crate::Command;
use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::{de::DeserializeOwned, Serialize};

use super::{run, TableAndSelectionOps};

#[derive(Debug, Clone)]
pub struct GetBuilder<T>(pub(crate) Command, pub(crate) Option<T>);

impl<T: Unpin + DeserializeOwned> GetBuilder<T> {
    pub(crate) fn new(primary_key: impl Serialize) -> Self {
        let args = Command::from_json(primary_key);
        let command = Command::new(TermType::Get).with_arg(args);

        Self(command, None)
    }

    pub async fn run(self, arg: impl run::Arg) -> crate::Result<Option<Option<T>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(self, arg: impl run::Arg) -> impl Stream<Item = crate::Result<Option<T>>> {
        self.0.into_arg::<()>().into_cmd().run::<_, Option<T>>(arg)
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T: Unpin + Serialize + DeserializeOwned> TableAndSelectionOps for GetBuilder<T> {
    type Parent = T;
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
