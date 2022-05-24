use crate::types::{Durability, ReturnChanges, WritingResponseType};
use crate::Command;
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;

pub struct UpdateBuilder(Command, UpdateOption);

// TODO finish this struct
#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct UpdateOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_atomic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

impl UpdateBuilder {
    pub fn new(document: &impl Serialize) -> Self {
        let args = Command::from_json(document);
        let command = Command::new(TermType::Update).with_arg(args);

        Self(command, UpdateOption::default())
    }

    pub async fn run(
        self,
        arg: impl super::run::Arg,
    ) -> crate::Result<Option<WritingResponseType>> {
        self.0
            .with_opts(self.1)
            .into_arg::<()>()
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

    pub fn with_non_atomic(mut self, non_atomic: bool) -> Self {
        self.1.non_atomic = Some(non_atomic);
        self
    }

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl Into<Command> for UpdateBuilder {
    fn into(self) -> Command {
        self.0
    }
}
