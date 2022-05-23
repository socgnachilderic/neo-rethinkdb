use crate::types::{Durability, ReturnChanges, WritingResponseType};
use crate::Command;
use futures::Stream;
use ql2::term::TermType;
use reql_rust_macros::CommandOptions;
use serde::Serialize;

pub struct InsertBuilder(Command, InsertOption);

// TODO finish this struct
#[derive(Debug, Clone, Copy, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct InsertOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
}

impl InsertBuilder {
    pub fn new(document: &impl Serialize) -> Self {
        let args = Command::from_json(document);
        let command = Command::new(TermType::Insert).with_arg(args);

        Self(command, InsertOption::default())
    }

    pub fn run(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<WritingResponseType>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
            .into_cmd()
            .run::<_, WritingResponseType>(arg)
    }

    pub fn with_durability(mut self, durability: Durability) -> Self {
        self.1.durability = Some(durability);
        self
    }

    pub fn with_return_changes(mut self, return_changes: ReturnChanges) -> Self {
        self.1.return_changes = Some(return_changes);
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
