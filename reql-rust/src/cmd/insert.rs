use crate::types::{Durability, ReturnChanges, WritingResponseType, Conflict};
use crate::{Command, Func};
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;

pub struct InsertBuilder(Command, InsertOption, Option<Func>);

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct InsertOption {
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

impl InsertBuilder {
    pub fn new(document: &impl Serialize) -> Self {
        let args = Command::from_json(document);
        let command = Command::new(TermType::Insert).with_arg(args);

        Self(command, InsertOption::default(), None)
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<WritingResponseType>> {
        let command = self.0;

        let command = if let Some(Func(func)) = self.2 {
            let args = func.with_opts(self.1);
            command.with_arg(args)
        } else {
            command.with_opts(self.1)
        };

        command.into_arg::<()>()
            .into_cmd()
            .run::<_, WritingResponseType>(arg)
            .try_next()
            .await
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

    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl Into<Command> for InsertBuilder {
    fn into(self) -> Command {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use crate::{cmd, r};
    use serde::Serialize;

    #[derive(Serialize)]
    struct Document<'a> {
        item: &'a str,
    }

    #[test]
    fn r_table_insert() {
        let query = r.table("foo").insert(&Document { item: "bar" });
        let serialised = cmd::serialise(&query.into());
        let expected = r#"[56,[[15,["foo"]],{"item":"bar"}]]"#;
        assert_eq!(serialised, expected);
    }
}
