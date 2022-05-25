use crate::Command;
use crate::types::Durability;
use crate::types::ReturnChanges;
use crate::types::WritingResponseType;
use futures::TryStreamExt;
use ql2::term::TermType;
use serde::Serialize;

pub struct DeleteBuilder(Command, DeleteOption);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct DeleteOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_changes: Option<ReturnChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_write_hook: Option<bool>,
}

impl DeleteBuilder {
    pub fn new() -> Self {
        let command = Command::new(TermType::Delete);
        Self(command, DeleteOption::default())
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

    #[doc(hidden)]
    pub fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl Into<Command> for DeleteBuilder {
    fn into(self) -> Command {
        self.0
    }
}
