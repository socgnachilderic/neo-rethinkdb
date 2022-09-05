use std::marker::PhantomData;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ops::ReqlOps;
use crate::types::{Conflict, Document, Durability, ReturnChanges, Sequence, WritingResponseType};
use crate::{Command, Func};

#[derive(Debug)]
pub struct InsertBuilder<T>(
    pub(crate) Command,
    pub(crate) InsertOption,
    pub(crate) Option<Func>,
    pub(crate) PhantomData<T>,
);

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct InsertOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<Conflict>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub conflict_func: Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

impl<T: Unpin + Serialize + DeserializeOwned> InsertBuilder<T> {
    pub(crate) fn new(document: &T) -> Self {
        let args = Command::from_json(document);
        let command = Command::new(TermType::Insert).with_arg(args);

        Self(command, InsertOption::default(), None, PhantomData)
    }

    pub(crate) fn new_many(documents: &[T]) -> Self {
        let args = Command::from_json(documents);
        let command = Command::new(TermType::Insert).with_arg(args);

        Self(command, InsertOption::default(), None, PhantomData)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<WritingResponseType<Sequence<Document<T>>>>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<WritingResponseType<Sequence<Document<T>>>>> {
        self.get_parent()
            .run::<_, WritingResponseType<Sequence<Document<T>>>>(arg)
    }

    pub fn with_durability(mut self, durability: Durability) -> Self {
        self.1.durability = Some(durability);
        self
    }

    pub fn with_return_changes(mut self, return_changes: ReturnChanges) -> Self {
        self.1.return_changes = Some(return_changes);
        self
    }

    pub fn with_ignore_write_hook(mut self, ignore_write_hook: bool) -> Self {
        self.1.ignore_write_hook = Some(ignore_write_hook);
        self
    }

    pub fn with_conflict(mut self, conflict: Conflict) -> Self {
        self.1.conflict = Some(conflict);
        self
    }

    pub fn with_conflict_func(mut self, func: Func) -> Self {
        self.2 = Some(func);
        self
    }

    #[doc(hidden)]
    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl<T> ReqlOps for InsertBuilder<T> {
    fn get_parent(&self) -> Command {
        let command = self.0.clone();

        let command = if let Some(Func(func)) = self.2.clone() {
            let args = func.with_opts(self.1);
            command.with_arg(args)
        } else {
            command.with_opts(&self.1)
        };

        command.into_arg::<()>().into_cmd()
    }
}

impl<T> Into<Command> for InsertBuilder<T> {
    fn into(self) -> Command {
        self.get_parent()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ReqlOps;
    use crate::{cmd, r};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Document {
        item: String,
    }

    #[test]
    fn r_table_insert() {
        let document = Document {
            item: "bar".to_string(),
        };

        let query = r.table::<Document>("foo").insert(&document);
        let serialised = cmd::serialise(&query.get_parent());
        let expected = r#"[56,[[15,["foo"]],{"item":"bar"}]]"#;
        assert_eq!(serialised, expected);
    }
}
