use std::time::Duration;

use futures::{Stream, TryStreamExt};
use ql2::term::TermType;
use serde::Serialize;

use crate::ops::ReqlOps;
use crate::types::{WaitFor, WaitResponseType};
use crate::Command;

#[derive(Debug, Clone)]
pub struct WaitBuilder(pub(crate) Command, pub(crate) WaitOption);

#[derive(Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd)]
#[non_exhaustive]
pub(crate) struct WaitOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<WaitFor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
}

impl WaitBuilder {
    pub(crate) fn new() -> Self {
        let command = Command::new(TermType::Wait);

        Self(command, WaitOption::default())
    }

    pub async fn run(self, arg: impl super::run::Arg) -> crate::Result<Option<WaitResponseType>> {
        self.make_query(arg).try_next().await
    }

    pub fn make_query(
        self,
        arg: impl super::run::Arg,
    ) -> impl Stream<Item = crate::Result<WaitResponseType>> {
        self.get_parent().run::<_, WaitResponseType>(arg)
    }

    pub fn with_wait_for(mut self, wait_for: WaitFor) -> Self {
        self.1.wait_for = Some(wait_for);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.1.timeout = Some(timeout.as_secs_f64());
        self
    }

    pub(crate) fn _with_parent(mut self, parent: Command) -> Self {
        self.0 = self.0.with_parent(parent);
        self
    }
}

impl ReqlOps for WaitBuilder {
    fn get_parent(&self) -> Command {
        self.0
            .clone()
            .with_opts(&self.1)
            .into_arg::<()>()
            .into_cmd()
    }
}

impl Into<Command> for WaitBuilder {
    fn into(self) -> Command {
        self.get_parent()
    }
}
